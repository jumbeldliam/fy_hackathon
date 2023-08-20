use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use chrono::prelude::*;
use uuid::Uuid;
use leptos::html::Input;
use serde::{Serialize, Deserialize};

const STORAGE_KEY: &str = "notes-app";

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);


    view! { cx,
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        <Title text="notes app"/>

        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {

    let user_ops = create_rw_signal(cx, User::new_user("will".to_owned()));

    view! { cx,
        <NotesHome user=user_ops/>
    }
}

struct SessionId(String);

impl SessionId {
    fn try_fetch() -> Option<Self> {
        todo!();
    }

    fn into_user(&self) -> User {
        todo!()
    }
}

#[component]
fn LandingPage(cx: Scope) -> impl IntoView {
    let (session_id, set_session_id) = create_signal(cx, SessionId::try_fetch());

    let sign_in_click = move |id| set_session_id.set(Some(id));

    view!{
        cx,
        <div> 

        </div>
    }
}

#[derive(PartialEq, Eq, Clone)]
pub struct Note {
    pub id: Uuid,
    pub title: RwSignal<String>,
    pub user: User,
    pub body: RwSignal<String>,
    pub date: TimeDate,
    pub last_edit: RwSignal<Option<TimeDate>>,
    pub pinned: RwSignal<bool>,
    pub maximized: RwSignal<bool>,
    pub minimized: RwSignal<bool>,
    pub is_editing: RwSignal<bool>,
    colour: String,
}

#[derive(PartialEq, Eq, Clone)]
pub struct User{
    username: String,
    /*pfp: todo!*/
    creation_date: TimeDate,
    //TODO: these should be replaced with Uuids to more easily iter through and filter the notes
    pinned_notes: Vec<Note>,
    minimized_notes: Vec<Note>,
    hidden_notes: Vec<Note>,
    uuid: Uuid,
    guest: bool
}

    
#[derive(PartialEq, Eq, Clone, Serialize, Deserialize)]
struct SerializedUser {
    username: String,
    creation_date: TimeDate,
    uuid: Uuid,
}

impl SerializedUser {
    fn into_normal(self) -> User {
        let SerializedUser {
            username,
            creation_date,
            uuid,
        } = self;

        User {
            username,
            creation_date,
            uuid,
            pinned_notes: Vec::new(),
            minimized_notes: Vec::new(),
            hidden_notes: Vec::new(),
            guest: false,
        }

    }
}

impl From<&User> for SerializedUser {
    fn from(from: &User) -> SerializedUser {
        let User {
            username,
            creation_date,
            uuid,
            ..
        } = from;

        SerializedUser {
            username: username.clone(),
            creation_date: creation_date.clone(),
            uuid: uuid.clone(),
        }
    }
}

impl User {
    fn new_user(username: String) -> Self {
        //TODO: need to check whether username exists within db
        User {
            username,
            creation_date: TimeDate::default(),
            pinned_notes: Vec::new(),
            minimized_notes: Vec::new(),
            hidden_notes: Vec::new(),
            uuid: Uuid::new_v4(),
            guest: false,
        }
    }

    fn new_guest() -> Self {
        User {
            username: String::new(),
            creation_date: TimeDate::default(),
            pinned_notes: Vec::new(),
            minimized_notes: Vec::new(),
            hidden_notes: Vec::new(),
            uuid: Uuid::new_v4(),
            guest: true,
        }
    }

    #[inline]
    fn is_guest(&self) -> bool {
        self.guest
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct TimeDate {
    epoch_time: i64,
}

impl TimeDate {
    const SECONDS_IN_WEEK: i64 = 7 * 24 * 60 * 60;

    pub fn format_date(&self) -> String {
        let local_date = chrono::Local.timestamp_opt(self.epoch_time, 0).unwrap().date_naive();
        local_date.to_string()
    }

    pub fn format_time(&self) -> String {
        let current_time = chrono::offset::Utc::now().timestamp();

        let comp_time = self.epoch_time;

        if current_time - comp_time > Self::SECONDS_IN_WEEK {
            return String::new()
        }

        let time = chrono::Local.timestamp_opt(comp_time, 0).unwrap();
        let minutes = time.minute();

        let minutes = if minutes < 10 {
            format!("0{}", minutes)
        }else{
            format!("{}", minutes)
        };

        format!{"{}:{}", time.hour(), minutes}
    }

    pub fn format_datetime(&self) -> String {
        let maybe_time = self.format_time();
        let date = self.format_date();


        if maybe_time.is_empty() {
            date
        }else{
            format!{"{} @ {}", date, maybe_time}
        }
    }

}

impl Default for TimeDate {
    fn default() -> Self {

        let local_time = chrono::offset::Local::now();
        let date = local_time.date_naive().to_string();
        let time = local_time.time();



        Self {
            epoch_time: chrono::offset::Utc::now().timestamp(),
        }
    }
}

fn get_new_pastel_colour() -> String {
    format!{"rgb({}, {}, {})", fastrand::u8(200..), fastrand::u8(200..), fastrand::u8(200..)}
}

impl Note {
    const TITLE_CHAR_LIMIT: u64 = 100;
    const BODY_CHAR_LIMIT: u64 = 1000;

    fn new_with_text(cx: Scope, title: String, user: &User, body: String) -> Self {
        let title = create_rw_signal(cx, title);
        let body = create_rw_signal(cx, body);

        let pinned = create_rw_signal(cx, false);
        let minimized = create_rw_signal(cx, false);
        let maximized = create_rw_signal(cx, false);

        let is_editing = create_rw_signal(cx, false);

        let last_edit = create_rw_signal(cx, None);

        Self {
            title,
            user: user.clone(),
            body,
            id: Uuid::new_v4(),
            date: TimeDate::default(),
            last_edit,
            pinned,
            maximized,
            minimized,
            is_editing,
            colour: get_new_pastel_colour()
        }
    }

    #[inline]
    fn new(cx: Scope, user: &User) -> Self {
        Self::new_with_text(cx, String::new(), user, String::new())
    }

    pub fn update_title(&self, new_title: String) {
        self.title.set(new_title);
        self.induce_edit();
    }

    pub fn update_body(&self, new_body: String) {
        self.body.set(new_body);
        self.induce_edit();
    }

    fn induce_edit(&self) {
        self.last_edit.set(Some(TimeDate::default()));
    }

    fn mirror_to_db(&self) {
        todo!()
    }

    fn toggle_pinned(&self) {
        self.pinned.update(|current_state| *current_state = !*current_state);
    }

    fn toggle_maximized(&self) {
        self.maximized.update(|maximized| *maximized = !*maximized);
    }

    fn unmaximize(&self) {
        self.maximized.set(false);
    }

    fn toggle_minimized(&self) {
        self.minimized.update(|minimized| *minimized = !*minimized);
    }

    fn toggle_edit(&self) {
        self.is_editing.update(|editing| *editing = !*editing);
    }

    fn unedit(&self) {
        self.is_editing.set(false);
    }

    fn hide(&self, user: &WriteSignal<User>) { 
        user.update(|usr| usr.pinned_notes.push(self.clone()));
    }

    //TODO: database mirroring
    fn delete(&self, notes: &WriteSignal<Vec<Note>>) {
        notes.update(|ns| ns.retain(|note| note != self))

    }

    fn add_pinned(&self, user: &WriteSignal<User>) {
        user.update(|usr| usr.pinned_notes.push(self.clone()));
    }

    fn remove_pinned(&self, user: &WriteSignal<User>) {
        user.update(|usr| usr.pinned_notes.retain(|note| note != self))
    }

    fn add_minimized(&self, user: &WriteSignal<User>) {
        user.update(|usr| usr.minimized_notes.push(self.clone()));
    }

    fn remove_minimized(&self, user: &WriteSignal<User>) {
        user.update(|usr| usr.minimized_notes.retain(|note| note != self));
    }
}

#[component]
fn PinButton(cx: Scope, toggle: WriteSignal<bool>) -> impl IntoView {
    let click = move || toggle.set(false);

    view!{
        cx,
        <button on:click=move |_| click()> "pin" </button>
    }
}

#[component]
fn ImgBtn<T: Fn() -> () + 'static>(cx: Scope, link: &'static str, on_click: T) -> impl IntoView {
    let link = link.to_owned();

    view!{
        cx,
        <button on:click=move |_| on_click() class="reset img_icon_wpr">
            <img class="reset img_icon" src=link/>
        </button>
    }
}

#[component]
fn ViewNote(
    cx: Scope, 
    note: ReadSignal<Note>, 
    viewer: RwSignal<User>,
    focused_view_note: RwSignal<Option<Note>>,
    write_notes: WriteSignal<Vec<Note>>,
    ) -> impl IntoView {



    //also, have transitions for hover state over the inital note
    //
    //add a tool bar with the name of the app maybe

    let pin_note = move || note.get().toggle_pinned();

    let minimize_note = move || note.get().toggle_minimized();

    let maximize_note = move || {
        note.get().toggle_maximized();
        focused_view_note.get().map(|prev_focused| (prev_focused != note.get()).then(|| prev_focused.unmaximize()));
        focused_view_note.set(Some(note.get().clone()))
    };

    let hide_note = move || note.get().hide(&viewer.write_only());

    let delete_note = move || note.get().delete(&write_notes);
    
    let check_for_edit_perms = move || if note.get().user.uuid == viewer.get().uuid {note.get().is_editing.set(true)};

    create_effect(cx, move |_| {
        if note.get().pinned.get() {
            /**
            write_notes.update(|notes| {
                let old_index = notes.iter().position(|n| n.id == note.get_untracked().id).unwrap();
                let mut index = old_index.clone();

                let is_minimized = note.get_untracked().minimized.get_untracked();

                if index != 0 {
                    index -= 1;
                    let comp_note = notes.get(index).unwrap();

                    if is_minimized {
                        while index > 0 && !comp_note.pinned.get_untracked() {
                            index += 1;
                        }

                    }else{
                        while index > 0 && !(comp_note.pinned.get_untracked() && !comp_note.minimized.get_untracked()) {
                            index +=1;
                        }
                    }
                    notes.swap(old_index, index);
                }
            });
            **/
            note.get().add_pinned(&viewer.write_only());
        }else{
            note.get().remove_pinned(&viewer.write_only());
        }
    });

    let note_body = view!{
        cx,
        <div class="flex_column">
            //{move || (note.get().pinned.get()).then_some(view!{cx, <PinButton toggle=note.get().pinned.write_only()/>})}

        {move || (note.get().pinned.get().then_some(view!{cx, 
            <div class="pinned_fold" style:background-color=move || format!("5px 5px color-mix(in srgb, {} 50%, rgba(0, 0, 0, .75))", note.get().colour)>
                <div class="pinned_mask"/>
            </div>
        }))}


            <div class="flex_seperator bottom_border">
                <ImgBtn link="https://cdn-icons-png.flaticon.com/512/2672/2672101.png" on_click= move || pin_note()/>
                {move || (note.get().minimized.get()).then_some(view!{cx, <h2 class="minimized_title_repr">{note.get().title.get()}</h2>})}
                <div>
                    <ImgBtn link="https://cdn-icons-png.flaticon.com/512/3484/3484290.png" on_click= move || minimize_note()/>
                    <ImgBtn link="https://cdn-icons-png.flaticon.com/512/2901/2901214.png" on_click= move || maximize_note()/>
                    {move || (!note.get().pinned.get() && note.get().user.uuid != viewer.get().uuid).then_some(view!{cx, <ImgBtn link="https://cdn-icons-png.flaticon.com/512/876/876769.png" on_click= move || hide_note()/>})}
                    {move || (note.get().user == viewer.get()).then_some(view!{cx, <ImgBtn link="https://cdn-icons-png.flaticon.com/512/3096/3096673.png" on_click=move || delete_note()/>})}
                </div>
            </div>

            <div class="flex_seperator text_padding">
                <h1 class="reset small_details"> {note.get().date.format_datetime()} </h1>
                <h1 class="reset small_details">"@"{note.get().user.username}</h1>
            </div>

            {move || (!note.get().minimized.get()).then_some(view!{cx, 
                <div class="text_bounding_area" on:dblclick=move |_| check_for_edit_perms()>
                    <h1 class="note_title"> {note.get().title} </h1>

                    <p class="note_text_body"> {note.get().body} </p>

                    {move || (note.get().last_edit.get().is_some()).then(|| {
                        let datetime = note.get().last_edit.get().unwrap();
                        view!{cx, <h1 class="previous_edit_timestamp">"last edit: "{datetime.format_datetime()}</h1>}
                    })}
                </div>
            })}
        </div>
    };

    view!{
        cx,
        {move || if note.get().maximized.get(){
            view!{
                cx,
                <div>
                    {note_body.clone()}
                </div>
            }
        }else{
            view!{
                cx,
                {note_body.clone()}
            }
        }}
    }
}

fn filter_input(inp: String) -> String {
    if inp.is_empty() {
        return inp;
    }

    inp.trim().to_owned()
}

fn parse_text_event_value(e: ev::Event) -> String {
    event_target_value(&e)
}


#[component]
fn EditNoteDisplay(cx: Scope, note: ReadSignal<Note>, viewer: RwSignal<User>, edit_focused_note: RwSignal<Option<Note>>, focused_view_note: RwSignal<Option<Note>>) -> impl IntoView {
    let update_title = move |e: ev::Event| {
        let str = filter_input(parse_text_event_value(e));
        note.get().update_title(str);
    };

    let update_body = move |e: ev::Event| {
        let str = filter_input(parse_text_event_value(e));
        note.get().update_body(str);
    };
    
    let body_update_notifier = create_rw_signal(cx, false);
    let title_update_notifier = create_rw_signal(cx, false);

    let (reached_title_limit, set_reached_title_limit) = create_signal(cx, note.get().body.get().len() as u64 > Note::TITLE_CHAR_LIMIT);
    let (reached_body_limit, set_reached_body_limit) = create_signal(cx, note.get().body.get().len() as u64 > Note::BODY_CHAR_LIMIT);

    create_effect(cx, move |_| {
        if !body_update_notifier.get() && !title_update_notifier.get() {
            note.get().mirror_to_db()
        }
    });

    let notifier_body = view!{
        cx,
        <CharacterLimit limit=Note::BODY_CHAR_LIMIT text=note.get().body reached_limit=set_reached_body_limit/>
    };

    //TOOD: css animations and stuff

    view!{
        cx,
        <div>
            <h1 class="text_edit_title"> "title" </h1>
            <input class="reset bottom_border single_line_text_input" type="text" value=note.get().title on:change=move |e| update_title(e)/>

            //{move || if title_update_notifier.get() {
                //Some(view!{
                    //cx, 
                    //{notifier_title.clone()}
                //})
            //}else{
                //None
            //}}
            <div>
                <h1 class="text_edit_body">"body" </h1>
                <textarea class="reset bottom_border multi_line_text_input" value=note.get().body on:change= move |e| update_body(e)  />
                //{move || if body_update_notifier.get() {
                    //Some(view!{
                        //cx, 
                        //{notifier_body.clone()}
                    //})
                //}else{
                    //None
                //}}
            </div>
        </div>
    };
}

#[component]
fn NoteEdit(cx: Scope, note: ReadSignal<Note>, viewer: RwSignal<User>, edit_focused_note: RwSignal<Option<Note>>, focused_view_note: RwSignal<Option<Note>>, write_notes: WriteSignal<Vec<Note>>) -> impl IntoView {
    let body_update_notifier = create_rw_signal(cx, false);
    let title_update_notifier = create_rw_signal(cx, false);

    let update_title = move |e: ev::Event| {
        let str = filter_input(parse_text_event_value(e));
        note.get().update_title(str);
    };

    let update_body = move |e: ev::Event| {
        body_update_notifier.set(true);
        let str = filter_input(parse_text_event_value(e.into()));
        note.get().update_body(str);
    };

    let update_title_from_keypress = move |e: ev::KeyboardEvent| {
        if e.key() == "Enter" {
            note.get().is_editing.set(false);
        }else{
            update_title(e.into())
        }
    };

    let update_body_from_keypress = move |e: ev::KeyboardEvent| {
        if e.key() == "Enter" {
            note.get().is_editing.set(false);
        }else{
            update_body(e.into())
        }
    };

    let (reached_body_limit, set_reached_body_limit) = create_signal(cx, note.get().body.get().len() as u64 > Note::BODY_CHAR_LIMIT);
    let (reached_title_limit, set_reached_title_limit) = create_signal(cx, note.get().title.get().len() as u64 > Note::TITLE_CHAR_LIMIT);


    let input_ref = create_node_ref::<Input>(cx);


    create_effect(cx, move |_| {
        if let Some(input) = input_ref.get() {
            request_animation_frame(move || {
                let _ = input.focus();
            })
        }
    });


    //let body_timeout_handle = create_rw_signal(cx, None);
    //let title_timeout_handle = create_rw_signal(cx, None);

    let notifier_body = view!{
        cx,
        <CharacterLimit limit=Note::BODY_CHAR_LIMIT text=note.get().body reached_limit=set_reached_body_limit/>
    };

        let notifier_title = view!{
        cx,
        <CharacterLimit limit=Note::TITLE_CHAR_LIMIT text = note.get().title reached_limit=set_reached_title_limit/>
    };

    view!{
        cx,
        <div>
            <h1 class="text_edit_title"> "title" </h1>
            <input type="text" class="reset bottom_border single_line_text_input" node_ref=input_ref placeholder=note.get().title value=note.get().title on:keydown=move |e| update_title_from_keypress(e.into()) on:input= move |e| update_title(e)/>
            //{move || title_update_notifier.get().then_some(view!{cx, <TimeoutNotifier body=notifier_title.clone() show_state=body_update_notifier timeout_duration=2000 timeout_handler=title_timeout_handle/>})}

            <div>
                <h1 class="text_edit_body"> "body" </h1>
                <textarea class="reset bottom_border multi_line_text_input" value=note.get().body placeholder=note.get().body on:keydown= move |e| update_body_from_keypress(e.into()) on:input= move |e| update_body(e)  />

                //{move || body_update_notifier.get().then_some(view!{cx, <TimeoutNotifier body=notifier_body.clone() show_state=body_update_notifier timeout_duration=2000 timeout_handler=body_timeout_handle/>})}
            </div>
        </div>
    }
}


#[component]
fn TimeoutNotifier(cx: Scope, body: View, show_state: RwSignal<bool>, timeout_duration: u64, timeout_handler: RwSignal<Option<TimeoutHandle>>) -> impl IntoView {
    create_effect(cx, move |_| {
        if show_state.get() {
            match timeout_handler.get() {
                Some(timeout) => {
                    show_state.set(true);
                    timeout.clear();

                    let new_timeout = Some(set_timeout_with_handle(move || {
                        show_state.set(false);
                        timeout_handler.set(None)
                    }, core::time::Duration::from_millis(timeout_duration)).unwrap());

                    timeout_handler.set(new_timeout);
                }
                _ => {
                    show_state.set(true);

                    let new_timeout = Some(set_timeout_with_handle(move || {
                        show_state.set(false);
                        timeout_handler.set(None)
                    }, core::time::Duration::from_millis(timeout_duration)).unwrap());

                    timeout_handler.set(new_timeout);
                }
            }
        }
    });

    view!{
        cx, 
        {move || (show_state.get()).then_some(view!{cx,  {body.clone()}})}
    }
}

pub enum TextColorState {
    Normal,
    Warning,
    Limit,
    OverLimit
}

impl ToString for TextColorState {
    fn to_string(&self) -> String {
        match *self {
            Self::Normal => "white".to_owned(),
            Self::Warning => "yellow".to_owned(),
            Self::Limit => "red".to_owned(),
            Self::OverLimit => "really red".to_owned(),
        }
    }
}

fn set_color_from_state(text_length: u64, text_limit: u64) -> TextColorState {
    if text_length > text_limit / 2 {
        TextColorState::Warning
    }else if text_length == text_limit {
        TextColorState::Limit
    }else if text_length >= text_limit {
        TextColorState::OverLimit
    }else{
        TextColorState::Normal
    }
}

#[component] 
fn CharacterLimit(cx: Scope, limit: u64, text: RwSignal<String>, reached_limit: WriteSignal<bool>) -> impl IntoView {
    let (color_state, set_color_state) = create_signal(cx, set_color_from_state(text.get().len() as u64, limit).to_string());

    create_effect(cx, move |_| {
        set_color_state.set(set_color_from_state(text.get().len() as u64, limit).to_string());

        if text.get().len() as u64 > limit {
            reached_limit.set(true)
        }else {
            reached_limit.set(false)
        }
    });

    view!{
        cx,
        <div>
            <h1>{text.get().len()} "/" {limit} </h1>
        </div>
    }
}

#[component]
fn EditButton(cx: Scope, edit_state: RwSignal<bool>) -> impl IntoView  {
    let toggle_edit = move || edit_state.update(|state| *state = !*state);

    let img_url = Signal::derive(cx, move || if edit_state.get() {
        "https://cdn-icons-png.flaticon.com/512/3395/3395544.png"
    }else{
        "https://cdn-icons-png.flaticon.com/512/860/860814.png"
    });

    view!{
        cx,
        <div class="edit_btn">
            <ImgBtn link=img_url.get() on_click=move || toggle_edit()/>
        </div>
    }
}

#[component]
fn Login(cx: Scope) -> impl IntoView {
    view!{
        cx,
        <div>
            <h1>"Login"</h1>


        </div>
    }
}

#[derive(PartialEq, Eq, Clone)]
enum NotesMode {
    Pinned,
    Queried(String),
    PinnedQueried(String),
    Normal,
}

#[derive(Serialize, Deserialize)]
struct SerializedNote {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub user: SerializedUser,
    pub date: TimeDate,
    pub last_edit: Option<TimeDate>,
}

impl SerializedNote {
    fn into_normal(self, cx: Scope) -> Note {

        let SerializedNote {
            id,
            title,
            body,
            user,
            date,
            last_edit,
        } = self;

        let title = create_rw_signal(cx, title);
        let body = create_rw_signal(cx, body);
        let last_edit = create_rw_signal(cx, last_edit);

        let pinned = create_rw_signal(cx, false);
        let maximized = create_rw_signal(cx, false);
        let minimized = create_rw_signal(cx, false);
        let is_editing = create_rw_signal(cx, false);

        let colour = get_new_pastel_colour();

        let user = user.into_normal();

        Note {
            id,
            title,
            body,
            user,
            date,
            last_edit,
            pinned,
            maximized,
            minimized,
            is_editing,
            colour,
        }
    }
}

impl From<&Note> for SerializedNote {
    fn from(from: &Note) -> SerializedNote {
        let Note {
            id,
            title,
            body,
            user,
            date,
            last_edit,
            ..
        } = from;

        SerializedNote {
            id: id.clone(),
            title: title.get(),
            body: body.get(),
            user: user.into(),
            date: date.clone(),
            last_edit: last_edit.get(),
        }
    }
}

//TODO: serde needs to be derived and figured out for this and then can store in localstorage


/**
fn get_stored_notes(cx: Scope) -> Vec<Note> {
    let starting_notes = if let Some(storage) = window().local_storage() {
        storage.get_item(STORAGE_KEY)
                .ok()
                .flatten()
                .and_then(|value| {
                    serde_json::from_str::<Vec<Note>>(&value).ok()
                })
                .map(|values| {
                    values
                        .into_iter()
                        .map(|stored| stored.into_todo(cx))
                        .collect()
                })
                .unwrap_or_default()
        } else {
            Vec::new()
        };
}
**/

#[component]
fn NotesHome(cx: Scope, user: RwSignal<User>) -> impl IntoView {
    let (notes, set_notes) = create_signal(cx, Vec::<Note>::new());

    let current_edit_focus = create_rw_signal(cx, None);
    let current_maximize_focus= create_rw_signal(cx, None);

    let (only_pinned, set_only_pinned) = create_signal(cx, false);

    provide_context(cx, set_notes);

    let filter_mode= create_rw_signal(cx, NotesMode::Normal);

    let filtered_notes = Signal::derive(cx, move || {
        notes.with(|notes| match filter_mode.get() {
            NotesMode::Pinned => notes.iter().filter(|note| note.pinned.get()).cloned().collect::<Vec<_>>(),
            NotesMode::Queried(query) => notes.iter().filter(|note| note.title.get().contains(&query)).cloned().collect::<Vec<_>>(),
            NotesMode::PinnedQueried(query) => notes.iter().filter(|note| note.title.get().contains(&query) && note.pinned.get()).cloned().collect::<Vec<_>>(),
            _ => notes.iter().cloned().collect::<Vec<_>>(),
        })
    });

    let add_note = move || set_notes.update(|cur_notes| cur_notes.push(Note::new(cx, &user.get())));

    create_effect(cx, move |_| {
        if only_pinned.get() {
            match filter_mode.get_untracked() {
                NotesMode::Normal => NotesMode::Pinned,
                NotesMode::Queried(s) => NotesMode::PinnedQueried(s),
                rest => rest
            }
        }else{
            match filter_mode.get_untracked() {
                NotesMode::Pinned => NotesMode::Normal,
                NotesMode::PinnedQueried(s) => NotesMode::Queried(s),
                rest => rest
            }
        }
    });

    let query_notes = move |e: ev::Event| {
        let value = event_target_value(&e);
        if value.is_empty() {
            filter_mode.set(NotesMode::Normal)
        }else {
            if only_pinned.get() {
                filter_mode.set(NotesMode::PinnedQueried(value))
            }else{
                filter_mode.set(NotesMode::Queried(value))
            }
        }
    };

    view!{
        cx,
        <div>
            <div class="spacer"/>
            <div class="handle_bar">
                <div class="handle_bar_show"/>
                <input type="text" class="reset note_search_bar" placeholder="search for notes..."  on:keydown=move |e| query_notes(e.into()) on:input= move |e| query_notes(e)/>
            </div>

            //<div>
            //    "filter only pinned notes"
            //    <input type="checkbox" on:click=move|_| set_only_pinned.update(|pinned| *pinned = !*pinned)/>
            //</div>


            {move || (!user.get().guest).then_some(view!{cx, <div class="add_note_btn"><ImgBtn link="https://cdn-icons-png.flaticon.com/512/1828/1828925.png" on_click=move || add_note() /></div>})}

            //{move || (!user.get().guest).then_some(view!{cx, <button on:click=move |_| add_note()> "create new note" </button> })}

            <DisplayNotes notes=filtered_notes user current_maximized=current_maximize_focus current_edit=current_edit_focus write_notes=set_notes/>
        </div>
    }
}

#[component]
fn DisplayNotes(cx: Scope, notes: Signal<Vec<Note>>, user: RwSignal<User>, current_maximized: RwSignal<Option<Note>>, current_edit: RwSignal<Option<Note>>, write_notes: WriteSignal<Vec<Note>>) -> impl IntoView {

    view!{
        cx,
        <div class="flex">
        <For 
            each=move || notes.get()
            key=|note| note.id
            view=move|cx, note: Note| {
                view!{
                    cx,
                    <DisplayNote viewer=user note current_maximized current_edit write_notes/>
                }
            }
        />            
        </div>

    }
}

#[derive(Clone, PartialEq)]
enum MinimumNoteHeight {
    Minimized, //85px
    Normal,
}

impl MinimumNoteHeight {
    fn into_px_height(&self) -> String {
        match *self {
            Self::Minimized => "85px".to_owned(),
            Self::Normal => "".to_owned(),
        }
    }
}

fn state_into_height(is_minimized: bool) -> MinimumNoteHeight {
    if is_minimized {
        MinimumNoteHeight::Minimized
    }else{
        MinimumNoteHeight::Normal
    }
}

#[component] 
fn DisplayNote(cx: Scope, viewer: RwSignal<User>, note: Note, current_maximized: RwSignal<Option<Note>>, current_edit: RwSignal<Option<Note>>, write_notes: WriteSignal<Vec<Note>>) -> impl IntoView {
    let (note, set_note) = create_signal(cx, note);

    //should have variable min-height whether or not the note is minimized


    let (min_note_height, set_min_note_height) = create_signal(cx, state_into_height(note.get().minimized.get()));

    create_effect(cx, move |_| {
        set_min_note_height.set(state_into_height(note.get().minimized.get()))
    });


    view!{
        cx,
        <div class="note_body" style:min-height={min_note_height.get().into_px_height()} style:background-color=move || note.get().colour style:box-shadow=move || format!("5px 5px color-mix(in srgb, {} 50%, rgba(0, 0, 0, .75))", note.get().colour)>
        {move || if note.get().is_editing.get() {
            view!{cx,
                <NoteEdit note viewer focused_view_note=current_maximized edit_focused_note=current_edit write_notes/>
            }
        }else{
            view!{cx, 
                <ViewNote note viewer focused_view_note=current_maximized write_notes/>
            }
        }}
            {move || (viewer.get() == note.get().user && !note.get().minimized.get()).then_some(view!{cx, <EditButton edit_state=note.get().is_editing />})}
        </div>
    }
}

/// 404 - Not Found
#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>(cx);
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { cx,
        <h1>"Not Found"</h1>
    }
}
