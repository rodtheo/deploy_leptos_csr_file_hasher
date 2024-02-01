use leptos::html::{Div, Input};
use leptos::logging::debug_warn;
use leptos::*;
use leptos_router::*;
use leptos_use::docs::{demo_or_body, BooleanDisplay};
use leptos_use::{use_drop_zone_with_options, UseDropZoneOptions, UseDropZoneReturn};
use std::error::Error;
use std::io::Read;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
// use wasm_bindgen_file_reader::WebSysFile;
use sha2::{Digest, Sha256};
// use std::rc::Rc;
use web_sys::*;

use async_std::prelude::*;
use async_std::task;
use indicatif::HumanBytes;
use std::cell::RefCell;
use std::rc::Rc;
use std::{thread, time};
// use futures::channel::mpsc::{channel, Receiver};

pub fn test() -> String {
    debug_warn!("Execucao test");
    format!("LOAD A FILE!")
}

// Define a constant for the desired buffer size
const BUFFER_SIZE: usize = 4096; // You can adjust this value based on your needs


#[component]
fn DemoArrayBuffer() -> impl IntoView {
    let (dropped, set_dropped) = create_signal(false);

    let (a, set_a) = create_signal(0);
    let (res, set_res) = create_signal(String::new());

    create_effect(move |_| {
        a.get();
        // let storage = gloo_storage::LocalStorage::raw();
        // sha256_file_sync(file.clone());
        let x = test();
        set_res.set(x);
    });

    let drop_zone_el = create_node_ref::<Div>();

    let UseDropZoneReturn {
        is_over_drop_zone,
        files,
    } = use_drop_zone_with_options(
        drop_zone_el,
        UseDropZoneOptions::default()
            .on_drop(move |_| {
                set_dropped(true);
                set_a.update(|value| *value += 1);
            })
            .on_enter(move |_| set_dropped(false)),
    );

    #[wasm_bindgen]
    // Helper function to get the next chunk of the file
    pub async fn get_next_chunk(file: &File, offset: usize) -> Option<web_sys::Blob> {
        let end = (offset + BUFFER_SIZE).min(file.size() as usize);

        if offset < end {
            debug_warn!("EXECUTANDO GET_NEXT_CHUNK: {:?}-{:?}", &offset, &end);
            let readed = file
                .slice_with_i32_and_i32(offset as i32, end as i32)
                .unwrap();

            let x: web_sys::js_sys::Promise = readed.array_buffer();

            let promise = js_sys::Promise::resolve(&x.into());
            let result = wasm_bindgen_futures::JsFuture::from(promise).await;

            Some(readed)
        } else {
            debug_warn!("NãããããããããaÕOOOOOOO GET_NEXT_CHUNK: {:?}", &offset);
            None
        }
    }

    view! {
        <div class="flex">
            <div class="w-full h-auto relative">
                <p>Drop files into dropZone</p>
                <img width="64" src="img/leptos-use-logo.svg" alt="Drop me"/>
                <div
                    node_ref=drop_zone_el
                    class="flex flex-col w-full min-h-[200px] h-auto bg-gray-400/10 justify-center items-center pt-6"
                >
                    <div>is_over_drop_zone: <BooleanDisplay value=is_over_drop_zone/></div>
                    <div>dropped: <BooleanDisplay value=dropped/></div>
                    <div class="flex flex-wrap justify-center items-center">
                        <For each=files key=|f| f.name() let:file>
                            <div class="w-200px bg-black-200/10 ma-2 pa-6">
                                <p>Name: {file.name()}</p>
                                <p>Size: {file.size()}</p>
                                <p>Type: {file.type_()}</p>
                                <p>Last modified: {file.last_modified()}</p>
                                <p>Result: {
                                    spawn_local(async move {
                                        a.get_untracked();

                                        let file_size = &file.size();

                                        let n_reading_rounds = ((file_size / (BUFFER_SIZE as f64)).ceil() as usize);
                                        debug_warn!("READING ROUNDS: {:?}", &n_reading_rounds);
    
                                        let mut hasher = RefCell::new(Sha256::new()); // Create a new SHA-2 hasher
                                        let mut n_iteration = 0 as usize;
                                      

                                        let onload_callback = Closure::wrap(Box::new(move |event: web_sys::Event| {

                                            n_iteration += 1;

                                            let reader = event
                                                .target()
                                                .unwrap()
                                                .dyn_into::<web_sys::FileReader>()
                                                .clone()
                                                .unwrap();
                                            let state = reader.ready_state();
                                            debug_warn!("Execucao STATE {:?}", &state);
                                            if reader.ready_state() == 2 {
                                                let result = reader.result().unwrap();
                                            let x = js_sys::Uint8Array::new(&result).to_vec();


                                            let mut mhasher = hasher.borrow_mut();
                                            mhasher.update(&x);

                                            if n_iteration == n_reading_rounds {

                                                let res_hash = mhasher.clone().finalize();
                                                let res_hash_out = format!("{}", hex::encode(&res_hash));
                                                set_res(res_hash_out.clone());
                                                debug_warn!("HASH FUNCIONANDO {:?} NA ITERAÇÃO {:?}", &res_hash_out, &n_iteration);

                                            }


                                            debug_warn!("Execucao FUNCIONANDO {:?}", &n_iteration);
                                            }

                                        }) as Box<dyn FnMut(_)>);


                                      
                                        let mut offset: usize = 0;


                                        // Start reading the file in chunks using a while let loop
                                        while let Some(slice) = get_next_chunk(&file, offset).await {
                                            let reader = FileReader::new().unwrap();
                                            reader.set_onload(Some(onload_callback.as_ref().unchecked_ref()));

                                            // Read the current chunk
                                            match reader.read_as_array_buffer(&slice) {
                                                Ok(_) => (),
                                                Err(e) => {


                                                    panic!("ERROR: {:?}", &e)},
                                            }



                                            // Update the offset for the next chunk
                                            offset += BUFFER_SIZE;

                                            let fs_to_moment = offset as u64;

                                            // Set the dynamic var to display the bytes readed
                                            set_res(HumanBytes(fs_to_moment).to_string());
                                        }




                                        // The closure will automatically be called when the reading is complete
                                        onload_callback.forget();
                                    });
                                    res
                                }</p>
                            </div>
                        </For>
                    </div>
                </div>
            </div>
        </div>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), || {
        view! { <DemoArrayBuffer /> }
    })
}
