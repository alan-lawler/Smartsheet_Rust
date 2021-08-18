mod json_structs;

pub mod processing {
    use reqwest::Response;
    use polars::prelude::{ Series, DataFrame, CsvWriter, SerWriter };
    use polars::series::NamedFrom;
    use std::error::Error;
    use std::io::{Write};
    use chrono::{DateTime, Utc};
    use std::fs::File;
    use crate::json_structs::structs::*;

    pub enum Endpoints {
        Users,
        Sheets,
        Events,
        Reports,
        Workspace,
    }

    pub async fn get(endpoint: Endpoints, position: Option<String>) -> Result<Response, Box<dyn Error>> {
        let events = format!("events?streamPosition={}&maxCount=10000", position.unwrap_or("".to_string()));
        let api = match endpoint {
            Endpoints::Users => "users?includeAll=True",
            Endpoints::Sheets => "sheets?includeAll=True",
            Endpoints::Events => events.as_str(),
            Endpoints::Reports => "reports?includeAll=True",
            Endpoints::Workspace => "workspaces?includeAll=True",
        };

        let url = "https://api.smartsheet.com/2.0/".to_owned() + api;

        let client = reqwest::Client::builder()
            .build()?;

        let resp = client
            .get(url)
            .bearer_auth("TOKEN")
            .header("Content-Encoding", "deflate")
            .send()
            .await?;
        Ok(resp)
    }

    pub async fn process_users(resp: Response) -> Result<DataFrame, Box<dyn Error>> {
        let user_data = resp.json::<Users>().await?;
        let total = user_data.total_count as usize;
        let mut user_id = vec![];
        let mut first_name = vec![];
        let mut last_name = vec![];
        let mut email = vec![];
        let mut i = 0;

        while i < total {
            user_id.push(user_data.data[i].id.to_string());
            first_name.push(user_data.data[i].first_name.as_ref().unwrap_or(&"".to_string()).to_owned());
            last_name.push(user_data.data[i].last_name.as_ref().unwrap_or(&"".to_string()).to_owned());
            email.push(user_data.data[i].email.to_owned());
            i += 1;
        }

        let s1 = Series::new("user_id", user_id);
        let s2 = Series::new("first_name", first_name);
        let s3 = Series::new("last_name", last_name);
        let s4 = Series::new("email", email);
        let df = DataFrame::new(vec![s1, s2, s3, s4])?;
        Ok(df)
    }

    pub async fn process_sheets(resp: Response) -> Result<DataFrame, Box<dyn Error>> {
        let sheet_data = resp.json::<Sheets>().await?;
        let total = sheet_data.total_count as usize;
        let mut sheet_id = vec![];
        let mut sheet_name = vec![];
        let mut i = 0;

        while i < total {
            sheet_id.push(sheet_data.data[i].id.to_string());
            sheet_name.push(sheet_data.data[i].name.to_owned());
            i += 1;
        }

        let s1 = Series::new("object_id", sheet_id);
        let s2 = Series::new("sheet_name", sheet_name);
        let df = DataFrame::new(vec![s1, s2])?;
        Ok(df)
    }

    pub async fn process_events(resp: Response) -> Result<DataFrame, Box<dyn Error>> {
        let event_data = resp.json::<Events>().await?;
        let total = event_data.data.len();
        let mut object_id = vec![];
        let mut object_type = vec![];
        let mut action = vec![];
        let mut time = vec![];
        let mut user_id = vec![];
        let mut source = vec![];
        // let mut org= vec![];  // used to add org name to final CSV file to be ingested by Splunk
        let mut i = 0;

        while i < total {
            object_id.push(event_data.data[i].object_id.to_string());
            object_type.push(event_data.data[i].object_type.to_string());
            action.push(event_data.data[i].action.to_owned());
            time.push(event_data.data[i].time.to_owned());
            user_id.push(event_data.data[i].user_id.to_string());
            source.push(event_data.data[i].source.to_owned());
            // org.push("aa".to_string());
            i += 1;
        }

        let s1 = Series::new("object_id", object_id);
        let s2 = Series::new("object_type", object_type);
        let s3 = Series::new("action", action);
        let s4 = Series::new("time", time);
        let s5 = Series::new("user_id", user_id);
        let s6 = Series::new("source", source);
        // let s7 = Series::new("organization", org);
        let df = DataFrame::new(vec![s1, s2, s3, s4, s5, s6])?;

        let next_position = event_data.next_position;
        let path = "position.txt";
        let mut output = File::create(path)?;
        write!(output, "{}", next_position)?;
        Ok(df)
    }

    pub async fn process_reports(resp: Response) -> Result<DataFrame, Box<dyn Error>> {
        let report_data = resp.json::<Reports>().await?;
        let total = report_data.total_count as usize;
        let mut report_id = vec![];
        let mut report_name = vec![];
        let mut i = 0;

        while i < total {
            report_id.push(report_data.data[i].id.to_string());
            report_name.push(report_data.data[i].name.to_owned());
            i += 1;
        }

        let s1 = Series::new("object_id", report_id);
        let s2 = Series::new("report_name", report_name);
        let df = DataFrame::new(vec![s1, s2])?;
        Ok(df)
    }

    pub async fn process_workspaces(resp: Response) -> Result<DataFrame, Box<dyn Error>> {
        let workspace_data = resp.json::<Workspaces>().await?;
        let total = workspace_data.total_count as usize;
        let mut workspace_id = vec![];
        let mut workspace_name = vec![];
        let mut i = 0;

        while i < total {
            workspace_id.push(workspace_data.data[i].id.to_string());
            workspace_name.push(workspace_data.data[i].name.to_owned());
            i += 1;
        }

        let s1 = Series::new("object_id", workspace_id);
        let s2 = Series::new("workspace_name", workspace_name);
        let df = DataFrame::new(vec![s1, s2])?;
        Ok(df)
    }

    pub fn write_csv(df: &DataFrame) {
        let now: DateTime<Utc> = Utc::now();
        let now = now.format("%Y-%m-%dT%H:%M:%S").to_string();
        let mut file = File::create(now + ".csv").expect("Error: Could not create file.");

        // write DataFrame to file
        CsvWriter::new(&mut file)
            .has_headers(true)
            .with_delimiter(b',')
            .finish(df)
            .expect("Failed to write file...");
    }
}