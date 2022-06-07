use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct PageTextConversion {
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NotebookMetadata {
    visible_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct NotebookContent {
    pages: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TextConversion {
    pub text: String,
    pub file_name: String,
    pub page_num: u16,
}

impl TextConversion {
    pub fn new(file_path: PathBuf) -> Option<Self> {
        if let Some(ext) = file_path.extension() {
            if ext == "json" {
                if let Some(parent) = file_path.parent() {
                    if let Some(parent_ext) = parent.extension() {
                        if parent_ext == "textconversion" {
                            let text = match get_page_text(file_path.to_owned()) {
                                Ok(text) => text,
                                Err(e) => {
                                    log::error!(
                                        "Unable to get page text for {:?}: {}",
                                        file_path,
                                        e
                                    );
                                    return None;
                                }
                            };

                            let file_name = match get_rm_file_name(file_path.to_owned()) {
                                Ok(text) => text,
                                Err(e) => {
                                    log::error!(
                                        "Unable to get associated text file name for {:?}: {}",
                                        file_path,
                                        e
                                    );
                                    return None;
                                }
                            };

                            let page_num = match get_page_number(file_path.to_owned()) {
                                Ok(text) => text,
                                Err(e) => {
                                    log::error!(
                                        "Unable to get page number of text for {:?}: {}",
                                        file_path,
                                        e
                                    );
                                    return None;
                                }
                            };

                            return Some(Self {
                                text,
                                file_name,
                                page_num,
                            });
                        }
                    }
                }
            }
        }

        None
    }
}

/// Parses the html text value into plain text from the json
/// text conversion file at this path
fn get_page_text(path: PathBuf) -> Result<String> {
    let file = std::fs::File::open(&path)?;
    let file_content: PageTextConversion = serde_json::from_reader(file)?;
    Ok(html2text::from_read(file_content.text.as_bytes(), 120))
}

/// Get the remarkable file name associated with the text conversion at this path
fn get_rm_file_name(path: PathBuf) -> Result<String> {
    match get_notebook_root(path) {
        Ok(nb_root) => {
            let file = std::fs::File::open(&format!("{}.metadata", nb_root))?;
            let file_content: NotebookMetadata = serde_json::from_reader(file)?;
            Ok(file_content.visible_name)
        }
        Err(e) => Err(anyhow!("Failed to get notebook root: {}", e)),
    }
}

/// Get page number of the text conversion at this path
///
/// Open up the associated ".content" file and parse the array of page ids, and then
/// return the index of the page id corresponding to this text conversion
fn get_page_number(path: PathBuf) -> Result<u16> {
    if let Some(page_identifier) = path.to_owned().file_stem() {
        if let Some(page_id) = page_identifier.to_str() {
            match get_notebook_root(path) {
                Ok(nb_root) => {
                    let file = std::fs::File::open(&format!("{}.content", nb_root))?;
                    let file_content: NotebookContent = serde_json::from_reader(file)?;
                    if let Some(page_number) =
                        file_content.pages.iter().position(|id| id == page_id)
                    {
                        return Ok(page_number.try_into().unwrap()); // Would panic if page is found at greater than 2^16 position....not too concerned...
                    }
                }
                Err(e) => anyhow::bail!("Failed to get notebook root: {}", e),
            }
        }
    }

    Err(anyhow!("Page number not accessible"))
}

/// Get the path to the root folder of the remarkable notebook
/// associated with this text conversion
fn get_notebook_root(path: PathBuf) -> Result<String> {
    if let Some(text_conv_folder) = path.parent() {
        if let Some(rm_notebook_root) = text_conv_folder.file_stem() {
            if let Some(rm_nb_root) = rm_notebook_root.to_str() {
                return Ok(format!(
                    "/home/root/.local/share/remarkable/xochitl/{}",
                    rm_nb_root
                ));
            }
        }
    }

    Err(anyhow!("Unable to get notebook root path"))
}
