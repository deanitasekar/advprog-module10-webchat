use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
use crate::services::event_bus::EventBus;

use crate::{User, services::websocket::WebsocketService};

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    wss: WebsocketService,
    messages: Vec<MessageData>,
    _producer: Box<dyn Bridge<EventBus>>,
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                )
                                .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    let value = input.value();
                    if !value.trim().is_empty() {
                        let message = WebSocketMessage {
                            message_type: MsgTypes::Message,
                            data: Some(value),
                            data_array: None,
                        };
                        if let Err(e) = self
                            .wss
                            .tx
                            .clone()
                            .try_send(serde_json::to_string(&message).unwrap())
                        {
                            log::debug!("error sending to channel: {:?}", e);
                        }
                        input.set_value("");
                    }
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);
        
        let on_keypress = ctx.link().batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                e.prevent_default();
                Some(Msg::SubmitMessage)
            } else {
                None
            }
        });
        
        html! {
            <div class="flex h-screen w-screen bg-gradient-to-br from-indigo-900 to-purple-800 overflow-hidden">
                // Sidebar
                <div class="flex-none w-80 h-full bg-white/10 backdrop-blur-sm border-r border-white/10 overflow-y-auto scrollbar-thin scrollbar-thumb-indigo-500 scrollbar-track-transparent">
                    <div class="sticky top-0 z-10 bg-indigo-800/80 backdrop-blur-sm p-4 border-b border-white/10">
                        <h2 class="text-xl font-bold text-white flex items-center">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6 mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
                            </svg>
                            {"Online Users"}
                        </h2>
                    </div>
                    <div class="p-3">
                        {
                            if self.users.is_empty() {
                                html! {
                                    <div class="flex items-center justify-center h-20 text-white/70 text-sm">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 mr-2 animate-spin" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                                        </svg>
                                        {"Waiting for users to connect..."}
                                    </div>
                                }
                            } else {
                                self.users.clone().iter().map(|u| {
                                    html!{
                                        <div class="flex items-center p-3 mb-2 rounded-lg bg-white/5 hover:bg-white/10 transition-all duration-200 cursor-pointer border border-white/5">
                                            <div class="relative">
                                                <img class="w-12 h-12 rounded-full object-cover border-2 border-indigo-400" src={u.avatar.clone()} alt="avatar"/>
                                                <div class="absolute bottom-0 right-0 w-3 h-3 bg-green-400 rounded-full border-2 border-indigo-800"></div>
                                            </div>
                                            <div class="ml-3">
                                                <div class="font-medium text-white">{u.name.clone()}</div>
                                                <div class="text-xs text-indigo-200">{"Online"}</div>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Html>()
                            }
                        }
                    </div>
                </div>
                
                // Main Chat Area
                <div class="flex-1 flex flex-col h-full">
                    // Chat Header
                    <div class="flex items-center h-16 bg-white/5 backdrop-blur-sm border-b border-white/10 px-6">
                        <div class="flex items-center">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-8 w-8 text-indigo-300" viewBox="0 0 20 20" fill="currentColor">
                                <path fill-rule="evenodd" d="M18 10c0 3.866-3.582 7-8 7a8.841 8.841 0 01-4.083-.98L2 17l1.338-3.123C2.493 12.767 2 11.434 2 10c0-3.866 3.582-7 8-7s8 3.134 8 7zM7 9H5v2h2V9zm8 0h-2v2h2V9zM9 9h2v2H9V9z" clip-rule="evenodd" />
                            </svg>
                            <h1 class="text-2xl font-bold text-white ml-3">{"YewChat"}</h1>
                        </div>
                        <div class="ml-auto">
                            <div class="bg-indigo-600 text-white px-4 py-2 rounded-full text-sm font-medium shadow-lg shadow-indigo-700/30">
                                {"Connected"}
                            </div>
                        </div>
                    </div>
                    
                    // Messages
                    <div class="flex-1 overflow-y-auto p-6 space-y-6 scrollbar-thin scrollbar-thumb-indigo-500 scrollbar-track-transparent">
                        {
                            if self.messages.is_empty() {
                                html! {
                                    <div class="flex flex-col items-center justify-center h-full text-white/60">
                                        <svg xmlns="http://www.w3.org/2000/svg" class="h-16 w-16 mb-4 text-indigo-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
                                        </svg>
                                        <p class="text-xl font-semibold mb-2">{"No messages yet"}</p>
                                        <p class="text-sm max-w-md text-center">{"Start the conversation by typing a message below."}</p>
                                    </div>
                                }
                            } else {
                                self.messages.iter().map(|m| {
                                    // Create a longer-lived fallback user profile as suggested by compiler
                                    let fallback_user = UserProfile {
                                        name: m.from.clone(),
                                        avatar: format!("https://avatars.dicebear.com/api/adventurer-neutral/{}.svg", m.from),
                                    };
                                    
                                    // Use reference to the longer-lived value
                                    let user = self.users.iter().find(|u| u.name == m.from).unwrap_or(&fallback_user);
                                    
                                    html!{
                                        <div class="flex items-start group">
                                            <div class="flex-shrink-0 mr-3">
                                                <img class="w-10 h-10 rounded-full object-cover border-2 border-indigo-400 group-hover:border-indigo-300 transition-all" src={user.avatar.clone()} alt="avatar"/>
                                            </div>
                                            <div class="flex-1">
                                                <div class="flex items-center mb-1">
                                                    <span class="font-semibold text-white mr-2">{user.name.clone()}</span>
                                                    <span class="text-xs text-white/50">{"just now"}</span>
                                                </div>
                                                <div class="bg-white/10 backdrop-blur-sm rounded-lg p-4 border border-white/5 max-w-2xl shadow-lg transform transition-all duration-200 group-hover:bg-white/15 group-hover:-translate-y-0.5">
                                                    {
                                                        if m.message.ends_with(".gif") {
                                                            html! {
                                                                <div class="mt-2 rounded-lg overflow-hidden border border-white/5">
                                                                    <img class="w-full" src={m.message.clone()} alt="GIF"/>
                                                                </div>
                                                            }
                                                        } else {
                                                            html! {
                                                                <p class="text-white">{ m.message.clone() }</p>
                                                            }
                                                        }
                                                    }
                                                </div>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Html>()
                            }
                        }
                    </div>
                    
                    // Message Input
                    <div class="p-4 bg-white/5 backdrop-blur-sm border-t border-white/10">
                        <div class="flex items-center bg-white/10 rounded-lg border border-white/10 px-4 py-2 focus-within:border-indigo-400 focus-within:ring-1 focus-within:ring-indigo-400 transition-all duration-200">
                            <input 
                                ref={self.chat_input.clone()} 
                                type="text" 
                                placeholder="Type your message..." 
                                class="bg-transparent border-none outline-none text-white w-full placeholder-white/50"
                                onkeypress={on_keypress} 
                            />
                            <button 
                                onclick={submit} 
                                class="ml-2 bg-indigo-600 hover:bg-indigo-700 text-white p-2 rounded-full transition-all duration-200 flex items-center justify-center shadow-lg shadow-indigo-700/30 transform hover:scale-105 active:scale-95"
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
                                    <path d="M10.894 2.553a1 1 0 00-1.788 0l-7 14a1 1 0 001.169 1.409l5-1.429A1 1 0 009 15.571V11a1 1 0 112 0v4.571a1 1 0 00.725.962l5 1.428a1 1 0 001.17-1.408l-7-14z" />
                                </svg>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}