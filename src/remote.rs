use chrono::DateTime;
use chrono::Utc;
use reqwest::Client;
use serde_derive::{
    Deserialize,
    Serialize,
};
use std::process::exit;

pub async fn get_dependency_url_remote(
    dependency_name: &String,
    dependency_version: &String,
) -> String {
    let url = format!(
        "{}/api/v1/revision-cli?project_name={}&revision={}",
        crate::BASE_URL,
        dependency_name,
        dependency_version
    );
    let req = Client::new().get(url);
    let get_project_response = req.send().await;
    match get_project_response {
        Ok(response) => {
            if response.status().is_success() {
                let response_text = response.text().await.unwrap();
                let revision = serde_json::from_str::<RevisionResponse>(&response_text);
                match revision {
                    Ok(revision) => {
                        if revision.data.is_empty() {
                            println!("Dependency not found, please check the dependency name");
                            exit(500);
                        }
                        return revision.data[0].clone().url;
                    }
                    Err(error) => {
                        println!("Error getting dependency {}", error.to_string());
                        exit(500);
                    }
                }
            } else {
                println!("Dependency not found, please check the dependency name");
                exit(500);
            }
        }
        Err(error) => {
            println!("Error getting dependency {}", error.to_string());
            exit(500);
        }
    }
}
//TODO clean this up and do error handling
pub async fn get_project_id(dependency_name: &String) -> String {
    let url = format!(
        "{}/api/v1/project?project_name={}",
        crate::BASE_URL,
        dependency_name
    );
    let req = Client::new().get(url);
    let get_project_response = req.send().await;

    match get_project_response {
        Ok(response) => {
            if response.status().is_success() {
                let response_text = response.text().await.unwrap();
                let project = serde_json::from_str::<ProjectResponse>(&response_text);
                match project {
                    Ok(project) => {
                        if project.data.is_empty() {
                            println!("Project not found, please check the dependency name (project name) or create a new project on https://soldeer.xyz");
                            exit(500);
                        }
                        return project.data[0].id.to_string();
                    }
                    Err(error) => {
                        println!("Error getting dependency {}", error.to_string());
                        exit(500);
                    }
                }
            } else {
                println!("Project not found, please check the dependency name (project name) or create a new project on https://soldeer.xyz");
                exit(500);
            }
        }
        Err(error) => {
            println!("Error getting dependency {}", error.to_string());
            exit(500);
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Revision {
    pub id: uuid::Uuid,
    pub version: String,
    pub internal_name: String,
    pub url: String,
    pub project_id: uuid::Uuid,
    pub deleted: bool,
    pub created_at: Option<DateTime<Utc>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Project {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub github_url: String,
    pub user_id: uuid::Uuid,
    pub deleted: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RevisionResponse {
    data: Vec<Revision>,
    status: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ProjectResponse {
    data: Vec<Project>,
    status: String,
}
