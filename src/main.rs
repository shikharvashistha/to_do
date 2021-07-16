//emscripten sdk is traditionally used to complie c and c++ code into javascript and webassembly.
//Here we'll compile rust code into javascript and webassembly.
//get emscripten sdk from https://kripken.github.io/emscripten-site/docs/getting_started/downloads.html
#[macro_use]//Helps us to get macros from inside of the crate
extern crate yew;//external crate

use yew::html::*;

struct Model {
    input: String,
    edit_input: String,
    todos: Vec<Todo>,
}
//added Todo Type for editing.
struct Todo {
    text: String,
    //edit field allows todo to be editted independently.
    edit: bool,
}

enum Msg {
    Add,
    Update(String),
    Remove(usize),
    //these three msg types are for editing todos.
    Edit(usize),
    UpdateEdit(String),
    Toggle(usize),
    RemoveAll,
    Nothing,
}

fn update(_: &mut Context<Msg>, model: &mut Model, msg: Msg) {
    match msg {
        Msg::Add => {
            let t = Todo {
                text: model.input.clone(),//what is in there in the input clone it
                edit: false,
            };
            model.todos.push(t);//put it in todos vector
            model.input = "".to_string();//replace with empty string in the input
        }
        Msg::Update(s) => {
            model.input = s;//replace with input string
        }
        Msg::Remove(i) => {
            model.todos.remove(i);//remove ith entry from todos vector
        }
        Msg::RemoveAll => {
            model.todos = vec![];//reset todos vector
        }
        Msg::UpdateEdit(s) => {
            //assigns the string s to edit_input.
            model.edit_input = s;
        }
        Msg::Edit(i) => {
            //creates a new todo from the edited text.
            let val = Todo {
                text: model.edit_input.clone(),
                edit: false,
            };
            model.todos.remove(i);
            model.todos.push(val);
        }
        Msg::Toggle(i) => {
            //gets todo from vector then looks at edit field.
            let todo = model.todos.get_mut(i).unwrap();
            todo.edit = !todo.edit;
        }
        Msg::Nothing => {}
    }
}

fn view(model: &Model) -> Html<Msg> {
    //allows for editing of todos independently.
    let view_todo_edit = |(i, todo): (usize, &Todo)| if todo.edit == true {
        //Closure to create a new todo from the edited text.
        html!{//This allow us to write html in rust.
            <label><input type="Text",
                    value=&todo.text,
                    oninput=|e: InputData| Msg::UpdateEdit(e.value),
                    onkeypress=move |e: KeyData| {
                        if e.key == "Enter" {Msg::Edit(i)} else {Msg::Nothing}
                    },
                    />
                    </label>
        }
    } else {
        html! {
            <label ondoubleclick=move|_| Msg::Toggle(i), > {format!("{} ", &todo.text)}
            </label>
        }
    };
    let view_todo = |(i, todo): (usize, &Todo)| {
        html!{//removes a particular todo from the todos vector.
            <li>
                { view_todo_edit((i, &todo))}
            </li>
            <button onclick = move |_| Msg::Remove(i),>{"X"}</button></li>
            //move closure takes the oewnership of our index
        }
    };


    html! {
        <div>
            <h1>{"Todo App"}</h1>
            <input
                placeholder="What do you want to do?",
                value=&model.input,//Signify that the value of our input is model.input
                oninput=|e: InputData| Msg::Update(e.value),//oninput listner it will take a closure which will take input data and msg update
                onkeypress=|e: KeyData| {//on key press listner allow user to enter actual data
                    if e.key == "Enter" {Msg::Add} else {Msg::Nothing}//if the key pressed is enter 
                    //we will put call the msg add function otherwise we do nothing
                },/>
                <p>{&model.input}</p>//paragraph tag to display the input string what user is typing    

        </div>
        <div>
            <button onclick = |_| Msg::RemoveAll, >{"Delete all Todos!"}</button>
            //button to delete all todos using onclick listner it'll take an element
            //which is anonoyms and it calls remove all function
        </div>
        <div>
            <ul>//unordered list
            {for model.todos.iter().enumerate().map(view_todo)}//enumerate across the iterator
            //it'll return numbers for each of the index of elements in the vector
            //then we'll map those numbers and values onto our view_todo closure.
            </ul>
        </div>
    }
}

fn main() {
    let model = Model {
        todos: vec![],
        input: "".to_string(),
        edit_input: "".to_string(),
    };//web server comes with hot reloading so everytime we change our code it will 
    //automatically recompile it on the server.
    program(model, update, view);
}