use walkdir::WalkDir;
extern crate globwalk;
use std::fs;
use glob::glob;


#[derive(Debug)]
pub enum ProjectError {
    ProjectNotFound,
}
#[derive(Debug)]
pub enum FolderError {
    NameAlreadyExists,
}
pub struct Project {
    folder: String,
    name: String
}

impl Project {
    pub fn create(folder: String) -> Result<Project, ProjectError> {
        Ok(Project { folder: folder ,
            name: String::from("blabla")})
    }

    pub fn get(project_folder: String) -> Option<Project> {




        Some(Project {
            folder: project_folder,
            name: String::from("blabla")
        })
    }

    pub fn get_folder(&self, path: Vec<String>, name: String) -> Option<Folder> {
        Folder::get(self, path, name)
    }

    pub fn add_folder(&mut self, path: Vec<String>, name: String) -> Result<Folder, FolderError> {
        Ok(Folder {
            path: path,
            name: name,
        })
    }
}

pub struct Folder {
    name: String,
    path: Vec<String>,
}

impl Folder {
    pub fn new(project: Project, path: Vec<String>, name: String) -> Folder {
        Folder {
            path: path,
            name: name,
        }
    }

    pub fn get(project: &Project, path: Vec<String>, name: String) -> Option<Folder> {
        Some(Folder { path, name })
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn get_project() {
        let project: Project = Project::get(String::from("../test_project"))
            .unwrap();

        assert_eq!(project.name, String::from("name"));
    }


    #[test]
    fn get_folder() {
        let folder: Option<Folder> = Project::get(String::from("../../test_project"))
            .unwrap()
            .get_folder(vec![String::from("notes")], String::from("subnotes"));

        assert_eq!(folder.unwrap().name, String::from("subnotes"));
    }

    // #[test]
    // fn list_folder_content() {
    //     let folder: Folder = Project::folder("../../test_project", vec!["notes", "subnotes"])?;

    //     assert_eq!(result, 4);
    // }
}
