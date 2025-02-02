use crate::PJError;
use home::home_dir;
use std::{fs, path::PathBuf};

fn folder_path() -> Result<PathBuf, PJError> {
    Ok(home_dir()
        .ok_or(PJError::FailedGetHome)?
        .join(".project_helper"))
}

/// return (tasks.json path, projects.json path)
/// 各種ファイル、フォルダが存在しない場合には作成.
pub fn initialize_helper_folder() -> Result<PathBuf, PJError> {
    let set_path = folder_path()?;
    let data_json_path = &set_path.join("data.json");
    // let projects_json_path = &set_path.join("projects.json");
    // let title_index_json_path = &set_path.join("title_index.json");

    if !set_path.exists() {
        fs::create_dir(&set_path).map_err(PJError::IoError)?;
    }
    if !data_json_path.exists() {
        fs::File::create(data_json_path).map_err(PJError::IoError)?;
    }
    // if !projects_json_path.exists() {
    //     fs::File::create(projects_json_path).map_err(PJError::IoError)?;
    // }
    // if !tasks_json_path.exists() {
    //     fs::File::create(title_index_json_path).map_err(PJError::IoError)?;
    // }

    Ok(data_json_path.to_path_buf())
}
