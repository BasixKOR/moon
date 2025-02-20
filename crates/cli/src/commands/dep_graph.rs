use crate::helpers::load_workspace;
use moon_runner::DepGraph;
use moon_task::Target;

pub async fn dep_graph(target_id: &Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let workspace = load_workspace().await?;
    let projects = workspace.projects;
    let mut graph = DepGraph::default();

    // Preload all projects
    projects.load_all()?;

    // Focus a target and its dependencies/dependents
    if let Some(id) = target_id {
        let target = Target::parse(id)?;

        graph.run_target(&target, &projects, &None)?;
        graph.run_target_dependents(&target, &projects)?;

    // Show all targets and actions
    } else {
        for project_id in projects.ids() {
            for task_id in projects.load(&project_id)?.tasks.keys() {
                graph.run_target(&Target::new(&project_id, task_id)?, &projects, &None)?;
            }
        }
    }

    println!("{}", graph.to_dot());

    Ok(())
}
