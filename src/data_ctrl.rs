use crate::PJHelpError;
use home::home_dir;
use std::{fs, path::PathBuf};

fn folder_path() -> Result<PathBuf, PJHelpError> {
    Ok(home_dir()
        .ok_or(PJHelpError::FailedGetHome)?
        .join(".project_helper"))
}

/// return (tasks.json path, projects.json path)
/// 各種ファイル、フォルダが存在しない場合に作成. 存在した場合はそのパスを返す.
pub fn initialize_helper_folder() -> Result<(PathBuf, PathBuf), PJHelpError> {
    let set_path = folder_path()?;
    let tasks_json_path = &set_path.join("tasks.json");
    let projects_json_path = &set_path.join("projects.json");

    if !set_path.exists() {
        fs::create_dir(&set_path).map_err(PJHelpError::IoError)?;
    }
    if !tasks_json_path.exists() {
        fs::File::create(tasks_json_path).map_err(PJHelpError::IoError)?;
    }
    if !projects_json_path.exists() {
        fs::File::create(projects_json_path).map_err(PJHelpError::IoError)?;
    }

    Ok((
        tasks_json_path.to_path_buf(),
        projects_json_path.to_path_buf(),
    ))
}

#[test]
fn t() {
    folder_path();
}
