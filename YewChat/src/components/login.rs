use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");
    let error = use_state(|| String::new());

    let oninput = {
        let current_username = username.clone();
        let error_state = error.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let value = input.value();
            current_username.set(value.clone());
            
            if value.len() < 3 && value.len() > 0 {
                error_state.set("Username should be at least 3 characters".to_string());
            } else {
                error_state.set(String::new());
            }
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };
    
    html! {
        <div class="min-h-screen bg-gradient-to-br from-indigo-900 to-purple-800 flex items-center justify-center w-full p-5">
            <div class="bg-white rounded-xl shadow-2xl max-w-md w-full p-8 transform transition-all duration-300 hover:scale-105">
                <div class="text-center mb-8">
                    <h1 class="text-3xl font-bold text-gray-800 mb-2">{"Welcome Back"}</h1>
                    <p class="text-gray-600">{"Sign in to continue to chat"}</p>
                </div>
                
                <div class="mb-6">
                    <label for="username" class="block text-sm font-medium text-gray-700 mb-2">{"Username"}</label>
                    <div class="relative">
                        <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                            <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5 text-gray-400" viewBox="0 0 20 20" fill="currentColor">
                                <path fill-rule="evenodd" d="M10 9a3 3 0 100-6 3 3 0 000 6zm-7 9a7 7 0 1114 0H3z" clip-rule="evenodd" />
                            </svg>
                        </div>
                        <input 
                            id="username"
                            type="text"
                            {oninput} 
                            class="pl-10 w-full rounded-lg border border-gray-300 p-4 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:border-transparent transition-all duration-200"
                            placeholder="Enter your username"
                        />
                    </div>
                    if !(*error).is_empty() {
                        <p class="mt-2 text-sm text-red-600">{&*error}</p>
                    }
                </div>

                <div class="mt-8">
                    <Link<Route> to={Route::Chat} classes="w-full block">
                        <button 
                            {onclick} 
                            disabled={username.len() < 3} 
                            class="w-full py-4 px-4 rounded-lg bg-indigo-600 text-white font-semibold transition-all duration-200 hover:bg-indigo-700 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            {"Start Chatting"}
                        </button>
                    </Link<Route>>
                </div>

                <div class="mt-6 text-center text-sm text-gray-500">
                    {"Need help? "}<a href="#" class="text-indigo-600 hover:text-indigo-800 font-medium">{"Contact support"}</a>
                </div>
            </div>
        </div>
    }
}