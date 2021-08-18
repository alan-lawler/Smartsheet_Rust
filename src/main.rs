use std::error::Error;
use std::fs::read_to_string;
use smartsheet::processing::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // get request to pull full list of users and save to df_users variable
    let users= get(Endpoints::Users, None).await?;
    let df_users = process_users(users).await?;

    // get request to pull full list of sheets and save to df_sheets variable
    let sheets = get(Endpoints::Sheets, None).await?;
    let df_sheets = process_sheets(sheets).await?;

    // get request to pull full list of reports and save to df_reports variable
    let reports = get(Endpoints::Reports, None).await?;
    let df_reports = process_reports(reports).await?;

    // get request to pull full list of workspaces and save to df_workspaces variable
    let workspaces = get(Endpoints::Workspace, None).await?;
    let df_workspaces = process_workspaces(workspaces).await?;

    // looks up last position in position.txt
    let position = read_to_string("position.txt")?;

    // get request to pull full list of events and save to df variable
    let events = get(Endpoints::Events, Some(position)).await?;
    let df = process_events(events).await?;

    // left joins all dataframes from above
    let df = df.left_join(&df_sheets, "object_id", "object_id")?;
    let df = df.left_join(&df_reports, "object_id", "object_id")?;
    let df = df.left_join(&df_workspaces, "object_id", "object_id")?;
    let df = df.left_join(&df_users, "user_id", "user_id")?;

    // writes df to disk in csv format to be ingested by Splunk
    write_csv(&df);

    Ok(())
}
