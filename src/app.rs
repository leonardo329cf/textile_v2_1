mod models;
mod pages;
mod services;
mod utils;

use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};
use wasm_bindgen::prelude::*;

use pages::{
    cutting_table::{CuttingTableItemPage, CuttingTableListPage},
    fabric::{FabricItemPage, FabricListPage},
    fabric_cut::FabricCutPage,
    home::HomePage,
    not_found::NotFoundPage,
    piece::{PieceItemPage, EditPieceItemPage}
};

use crate::app::pages::{generate_gcode::GenerateGCodePage, export_disposition::ExportDispositionPage};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], catch)]
    pub async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: &str);
}

#[component]
pub fn App<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        header { AppNav {} }
        main { AppRouter {} }
        footer(class="mt-auto has-background-black has-text-centered") {
            figure(class="image is-inline-block") {
                img(src="images/logo.png") {}
            }
        }
    }
}

#[component]
fn AppNav<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        nav(id="navbar", class="navbar mb-2", role="navigation", aria-label="main navigation") {
            div(id="navMenu", class="navbar-menu") {
                div(class="navbar-start") {
                    a(class="navbar-item", href="/") { "Sobre" }
                    a(class="navbar-item", href="/fabric") { "Tecidos" }
                    a(class="navbar-item", href="/cutting-table") { "Mesas de corte" }
                    a(class="navbar-item", href="/fabric-cut") { "Cortes" }
                }
            }
        }
    }
}

#[component]
fn AppRouter<G: Html>(cx: Scope) -> View<G> {
    view! { cx,
        Router(
            integration=HistoryIntegration::new(),
            view=|cx, route: &ReadSignal<AppRoutes>| {
                view! { cx, (
                    match route.get().as_ref() {
                        AppRoutes::Index => view! { cx, HomePage {} },
                        AppRoutes::NotFound => view! { cx, NotFoundPage {} },
                        AppRoutes::FabricList => view! { cx, FabricListPage {} },
                        AppRoutes::FabricItem(id) => {
                            view! {cx, FabricItemPage(id = *id) {}}
                        },
                        AppRoutes::CuttingTableList => view! { cx, CuttingTableListPage {} },
                        AppRoutes::CuttingTableItem(id) => {
                            view! {cx, CuttingTableItemPage(id = *id) {}}
                        },
                        AppRoutes::FabricCut => view! { cx, FabricCutPage {} },
                        AppRoutes::PieceItem => {
                            view! {cx, PieceItemPage {}}
                        },
                        AppRoutes::EditPieceItem(piece_type_id, id) => view! {
                            cx, EditPieceItemPage(
                                piece_type_id = *piece_type_id,
                                id = *id
                            ) {}
                        },
                        AppRoutes::GenerateGCode => view! {
                            cx, GenerateGCodePage {}
                        },
                        AppRoutes::ExportDisposition => view!(
                            cx, ExportDispositionPage {}
                        )
                    }
                )}
            }
        )
    }
}

#[derive(Route)]
enum AppRoutes {
    #[to("/")]
    Index,
    #[not_found]
    NotFound,
    #[to("/fabric")]
    FabricList,
    #[to("/fabric/<_>")]
    FabricItem(i32),
    #[to("/cutting-table")]
    CuttingTableList,
    #[to("/cutting-table/<_>")]
    CuttingTableItem(i32),
    #[to("/fabric-cut")]
    FabricCut,
    #[to("/piece-item")]
    PieceItem,
    #[to("/edit-piece-item/<_>/<_>")]
    EditPieceItem(i32, u32),
    #[to("/generate-g-code")]
    GenerateGCode,
    #[to("/export-disposition")]
    ExportDisposition,
}
