use dioxus::{
    document::{self, Eval},
    prelude::*
};

use crate::backend::{
    playground_client::TcpClient,
    playground_lib::*
};

use std::time::Duration;
use async_std::task;



const CSS: Asset = asset!("/assets/styling/playground.css");

const DEFAULT_FN: &str = r#"fn main() {
    println!("Hello World!");
}"#;



fn js_append_out(info: JsonInfo) -> Eval {
    let style: String;
    let classes: String;
    let txt: String;

    match info.header.as_str() {
        "exit" => {
            classes = "".to_string();
            txt = "exit".to_string()+&info.body;
        },
        "error" => {
            classes = "err".to_string();
            txt = "----- ERROR -----\n".to_string()+&info.body;
        },
        "stderr" => {
            classes = "exterr".to_string();
            txt = info.body;
        },
        "stdout" => {
            classes = "".to_string();
            txt = info.body;
        },
        "compilation_result" => {
            classes = "complog".to_string();
            txt = info.body;
        },
        "" => {
            classes = "".to_string();
            txt = info.body;
        },
        _ => {
            classes =  "".to_string();
            txt = format!("{}:\n{}",info.header.to_uppercase(), &info.body);
        }
    }

    let out = txt
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('$', "\\$")
        .replace("\n", "<br/>")
        .replace("\r", "")
        .replace('\t', "&nbsp;");
    
    if !classes.is_empty() {
        style = format!("class='{classes}'");//TODO
    } else {
        style = "".to_string();
    }

    document::eval(
        &format!(
            r#"
                let pgd_out = document.getElementById("playground_file_output");
                pgd_out.insertAdjacentHTML("beforeend", "<div {style}>{out}</div>");
                return "Pgd output now modifyed in: "+pgd_out.innerHTML;
            "#,
        )
    )
}

fn js_get_in(target: &str) -> Eval {
    document::eval(
        &format!(
            r#"return document.getElementById("{target}").value;"#,
        )
    )
}

fn js_clear_in(target: &str) -> Eval {
    document::eval(
        &format!(
            r#"
                document.getElementById("{target}").innerHTML = '';
                document.getElementById("{target}").value = '';
                return 'OK';
            "#,
        )
    )
}

#[component]
pub fn Playground() -> Element {
    let mut run_compile_clicked = use_signal(|| false);//trace run & compile button state
    let mut shutdown_client = use_signal(|| false);
    let mut client = use_signal(|| TcpClient::init_as_none());

    //define run & compile function
    let run_compile = move || {
        spawn(async move {
            match js_clear_in("playground_file_output").await {//clear text
                Ok(_msg) => {},
                Err(err) => eprintln!("{err}"),
            }
            
            match js_get_in("playground_file_input").await {//get playground_file_input' value
                Ok(input) => {
                    match input.as_str() {
                        Some(input) => {
                            if let Ok(temp_new_client) = TcpClient::spawn("127.0.0.1:8000") {//spawn client
                                {
                                    client.set(temp_new_client);
                                    client.write().send_run_compile_req(input.to_string());
                                }
                    
                                let mut _read_res = Ok(None);
                                
                                loop {
                                    { _read_res = client.write().read(); }//read from stream
                                    match _read_res {
                                        Ok(None) => {},
                                        Ok(jsoninfo) => {
                                            if let Some(info) = jsoninfo {
                                                if info.header == "exit" {
                                                    break;
                                                }
                    
                                                if let Err(err) = js_append_out(info).await {
                                                    eprintln!("{err}");
                                                } 
                                            }
                                            
                                        },
                                        Err(err) => {
                                            let msg = format!(
                                                "----- FATAL ERROR -----\n{}", err
                                            );

                                            if let Err(err) = js_append_out(
                                                JsonInfo::from_string("error".to_string(), msg)
                                            ).await {
                                                eprintln!("{err}");
                                            }
                                            break;
                                        },
                                    }

                                    if *shutdown_client.read() {
                                        { client.write().shutdown(); }
                                        break;
                                    }
                    
                                    task::sleep(Duration::from_millis(500)).await;
                                }
                            }
                        },
                        None => {
                            js_append_out(JsonInfo::from_string(//append text to output
                                "error".to_string(),
                                format!(
                                    "{}{}",
                                    "Could not send input: ",
                                    "element with 'playground_file_input' id has a incomprehensible value!"
                                )
                            ));
                        },
                    }
                },
                Err(err) => {
                    js_append_out(JsonInfo::from_string(//append text to output
                        "error".to_string(),
                        format!("Could not send input: {err}")
                    ));
                },
            };
            
            run_compile_clicked.set(false);//report end request (toggle now in RUN & COMPILE)
        });
    };

    let send_input = move || {
        spawn(async move {
            match js_get_in("playground_stdin").await {//get 'playground_stdin' value
                Ok(input) => {
                    match input.as_str() {
                        Some(input) => {
                            client.write().send_input_req(input.to_string());//send input to stream
                        },
                        None => {
                            js_append_out(JsonInfo::from_string(
                                "error".to_string(),
                                format!(
                                    "{}{}",
                                    "Could not send input: ",
                                    "element with 'playground_stdin' id has a incomprehensible value!"
                                )
                            ));
                        },
                    }
                },
                Err(err) => {
                    js_append_out(JsonInfo::from_string(
                        "error".to_string(),
                        format!("Could not send input: {err}")
                    ));
                },
            };
        });
    };

    rsx! {
        link {
            rel: "stylesheet",
            href: CSS,
        }
        div { id: "main_div", class: "playground-container",
            h1 { "Rust Playground" }

            div { id: "playground_editor",
                textarea {
                    id: "playground_file_input",
                    name: "playground_file_input",
                    class: "editor-textarea",
                    "{DEFAULT_FN}"
                }
                button {
                    id: "playground_file_input_btn",
                    class: "run-btn",
                    onclick: move |_| {
                        if *run_compile_clicked.read() {
                            shutdown_client.set(true);
                        } else {
                            shutdown_client.set(false);
                            run_compile_clicked.set(true);
                            run_compile();
                        }
                    },
                    match *run_compile_clicked.read() {
                        true => "◼ Stop",
                        false => "▶ Compile & Run",
                    }
                }
                
                    code {
                        id: "playground_file_output",
                        
                        alt:"Output",
                        class: "output-box"
                    }
                
            }

                div { class: "command-box",
                    input {
                        id: "playground_command",
                        name: "playground_command",
                        r#type: "text",
                        placeholder: "Run cargo commands (e.g., cargo check)",
                        class: "command-input"
                    }
                    button {
                        id: "playground_command_btn",
                        onclick: move |_| {
                            spawn(async move {
                                match js_get_in("playground_command").await {
                                    Ok(cmd) => {
                                        if let Some(command) = cmd.as_str() {
                                            client.write().send_input_req(format!("__CMD__{}", command));
                                        }
                                    },
                                    Err(err) => {
                                        js_append_out(JsonInfo::from_string("error".to_string(), format!("Command error: {err}")));
                                    }
                                }
                            });
                        },
                        "▶ Run Command"
                    }
                }
            
        }
    }
}