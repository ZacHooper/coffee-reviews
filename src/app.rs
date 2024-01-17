use crate::models::{Coffee, FormData};
use leptos::ev::SubmitEvent;
use leptos::logging::log;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use std::collections::HashMap;
use wasm_bindgen_futures::spawn_local;

/// This is a generic function that can be used to set the value of a form number input.
/// This function will parse the value of the event target into an i32 and then call the
/// provided handler with that value.
///
/// # Example
///
/// ```rust
/// let (value, set_value) = create_signal("".to_string());
/// !view! {
///    <input value={value} on:input=create_number_input_handler(set_value) />
/// }
/// ```
///
/// # Panics
///
/// This function will panic if the value cannot be parsed into the type of the signal
fn create_number_input_handler<T, F>(handler: F) -> impl Fn(T) + 'static
where
    T: wasm_bindgen::JsCast,
    F: Fn(i32) + 'static,
{
    move |ev: T| {
        let value = event_target_value(&ev).parse::<i32>().unwrap_or(0);
        handler(value);
    }
}

#[component]
pub fn SelectOption(is: &'static str, value: ReadSignal<String>) -> impl IntoView {
    view! {
        <option value=is selected=move || value.get() == is>{is}</option>
    }
}

#[component]
pub fn Select(
    id: &'static str,
    children: Children,
    on_change: WriteSignal<String>,
) -> impl IntoView {
    view! {
        <select
            id=id
            class="p-2 bg-coffee-lightest rounded-sm"
            on:change=move |ev| on_change(event_target_value(&ev))
        >
            {children()}
        </select>
    }
}

#[component]
pub fn NumberInput(
    id: &'static str,
    value: ReadSignal<i32>,
    min: i32,
    max: i32,
    step: i32,
    on_input: WriteSignal<i32>,
) -> impl IntoView {
    view! {
        <input
            id=id
            type="number"
            class="p-2 bg-coffee-lightest rounded-sm"
            min=min
            max=max
            step=step
            value={value}
            prop:value=value
            on:input=create_number_input_handler(on_input)
        />
    }
}

#[component]
pub fn Label(for_: &'static str, children: &'static str) -> impl IntoView {
    view! {
        <label for=for_ class="text-right self-center">{children}</label>
    }
}

#[component]
fn Home() -> impl IntoView {
    // Form Inputs
    let (brew_method, set_brew_method) = create_signal("pour-over".to_string());
    let (coffee, set_coffee) = create_signal("ocean_grind".to_string());
    let (weight, set_weight) = create_signal(30);
    let (water, set_water) = create_signal(500);
    let (grind_size, set_grind_size) = create_signal(12);
    let (temperature, set_temperature) = create_signal(95);
    let (rating, set_rating) = create_signal(8);
    let (funkiness, set_funkiness) = create_signal(8);
    let (acidity_bitterness, set_acidity_bitterness) = create_signal(5);
    let (strength, set_strength) = create_signal(5);
    let (notes, set_notes) = create_signal("".to_string());

    let mut coffees = HashMap::new();
    coffees.insert(
        "Ocean Grind".to_string(),
        Coffee {
            name: "Single origin".to_string(),
            roastery: "Ocean Grind".to_string(),
            roast_date: "2024-01-01".to_string(),
            roast_level: "Medium".to_string(),
            origin: "Guatemala".to_string(),
            region: "El Boqueron, Santa Rosa".to_string(),
            farm: "Unknown".to_string(),
            altitude: 0.0,
            variety: "Unknown".to_string(),
            processing_method: "Natural".to_string(),
            tasting_notes: "Red wine, cooked orange, hazelnut, chocolate".to_string(),
            nickname: "Ocean Grind - Single origin".to_string(),
        },
    );
    coffees.insert(
        "Seven Seeds".to_string(),
        Coffee {
            name: "Habtamu Fekadu".to_string(),
            roastery: "Seven Seeds".to_string(),
            roast_date: "2023-12-23".to_string(),
            roast_level: "Light".to_string(),
            origin: "Ethiopia".to_string(),
            region: "Chelbessa".to_string(),
            farm: "Worka Chelbessa local farmers".to_string(),
            altitude: 2186.5,
            variety: "74110,74112".to_string(),
            processing_method: "Fully Washed".to_string(),
            tasting_notes: "Peach, Limonata, Apricot".to_string(),
            nickname: "Seven Seeds - Habtamu Fekadu".to_string(),
        },
    );

    // let brew_method: NodeRef<Input> = create_node_ref();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let selected_coffee = match coffees.get(&coffee.get()) {
            Some(c) => c,
            None => panic!("Coffee not found"),
        };

        let form = FormData {
            brew_method: brew_method.get(),
            coffee: selected_coffee.clone(),
            weight: weight.get(),
            water: water.get(),
            grind_size: grind_size.get(),
            temperature: temperature.get(),
            rating: rating.get(),
            funkiness: funkiness.get(),
            acidity_bitterness: acidity_bitterness.get(),
            strength: strength.get(),
            notes: notes.get(),
        };
        log!("form: {:?}", form);

        spawn_local(async move {
            let client = reqwest::Client::new();
            let url = "https://popxtqhuczfmrct6dfpboxw5qm0xvbta.lambda-url.ap-southeast-2.on.aws/";
            let json = serde_json::json!({
                "brew_method": form.brew_method,
                "coffee": form.coffee,
                "weight": form.weight,
                "water": form.water,
                "grind_size": form.grind_size,
                "temperature": form.temperature,
                "rating": form.rating,
                "funkiness": form.funkiness,
                "acidity_bitterness": form.acidity_bitterness,
                "strength": form.strength,
                "notes": form.notes
            });

            let res = client.post(url).json(&json).send().await.unwrap();

            log!("response: {:?}", res);

            let res_body: String = res.text().await.unwrap();
            log!("response body: {:?}", res_body);
        });
    };

    view! {
        <main class="bg-coffee-lightest h-full min-h-screen w-full grid">
            <div class="
            bg-coffee-white text-coffee-black
            max-w-3xl h-max rounded-xl px-16
            justify-self-center self-center">
                <h1 class="p-6 text-3xl text-center">"Coffee Review"</h1>

                // Form
                <form on:submit=on_submit class="grid grid-cols-[max-content_1fr] gap-y-1 gap-x-4 ">
                    // <label for="brew-method" class="text-right">"Brew Method: "</label>
                    <Label for_="brew-method" children="Brew Method: "/>
                    <Select id="brew-method" on_change=set_brew_method>
                        <SelectOption value=brew_method is="Pour Over"/>
                        <SelectOption value=brew_method is="Aeropress"/>
                        <SelectOption value=brew_method is="Cold Brew"/>
                    </Select>

                    // todo. add new coffee option
                    <Label for_="coffee" children="Coffee: "/>
                    <Select id="coffee" on_change=set_coffee>
                        <SelectOption value=coffee is="Ocean Grind"/>
                        <SelectOption value=coffee is="Seven Seeds"/>
                    </Select>

                    <Label for_="weight" children="Weight (g): "/>
                    <NumberInput id="weight" value=weight on_input=set_weight min=0 max=1000 step=1/>

                    <Label for_="water" children="Water (g): "/>
                    <NumberInput id="water" value=water on_input=set_water min=0 max=1000 step=5/>

                    <Label for_="grind-size" children="Grind Size (1-30): "/>
                    <NumberInput id="grind-size" value=grind_size on_input=set_grind_size min=0 max=30 step=1/>

                    <Label for_="temperature" children="Temperature (°C): "/>
                    <NumberInput id="temperature" value=temperature on_input=set_temperature min=50 max=120 step=1/>

                    <Label for_="rating" children="Rating (1-10): "/>
                    <NumberInput id="rating" value=rating on_input=set_rating min=0 max=10 step=1/>

                    <Label for_="funkiness" children="Funkiness (1-10): "/>
                    <NumberInput id="funkiness" value=funkiness on_input=set_funkiness min=0 max=10 step=1/>

                    <Label for_="acidity_bitterness" children="Acidity/Bitterness (1-10): "/>
                    <input id="acidity_bitterness" type="range" value="5" min="1" max="10" step="1" class="p-2"
                        prop:value=acidity_bitterness
                        on:input=create_number_input_handler(set_acidity_bitterness)
                    />

                    <Label for_="strength" children="Strength (1-10): "/>
                    <input id="strength" type="range" value="5" min="1" max="10" step="1" class="p-2"
                        prop:value=strength
                        on:input=create_number_input_handler(set_strength)
                        class="p-2 accent-coffee-lightest bg-coffee-white hover:bg-coffee-light"
                    />

                    <Label for_="notes" children="Notes: "/>
                    <textarea id="notes" rows="4" cols="50"
                        on:input=move |ev| set_notes(event_target_value(&ev))
                        class="p-2 bg-coffee-lightest rounded-sm"
                    >
                        {untrack(move || notes.get())}
                    </textarea>

                    <button type="submit">"Submit"</button>
                </form>
            </div>
        </main>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        // <Stylesheet id="leptos" href="/pkg/coffee-reviews.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path="" view=Home/>
            </Routes>
        </Router>
    }
}
