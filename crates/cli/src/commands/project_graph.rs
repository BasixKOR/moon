use crate::helpers::load_workspace;

pub async fn project_graph(project_id: &Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let workspace = load_workspace().await?;

    if let Some(id) = project_id {
        workspace.projects.load(id)?;
    } else {
        workspace.projects.load_all()?;
    }

    println!("{}", workspace.projects.to_dot());

    Ok(())
}
