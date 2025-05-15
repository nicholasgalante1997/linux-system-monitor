pub mod routes {

    use crate::{app::AppState, models::system_report::SystemReporter, ui::HttpViews};
    use actix_web::{HttpResponse, Responder, get, web};
    use anyhow::Error;
    use futures::{future::ok, stream::once};

    #[get("/http-views/pages/overview")]
    pub async fn render_overview_as_html(app_state: web::Data<AppState>) -> impl Responder {
        let mut components = app_state.components.lock().unwrap();
        let mut disks = app_state.disks.lock().unwrap();
        let mut networks = app_state.networks.lock().unwrap();
        let mut system = app_state.system.lock().unwrap();

        let mut system_reporter =
            SystemReporter::new(&mut components, &mut disks, &mut networks, &mut system);
        let system_report = system_reporter.build_report();

        let view = HttpViews::get_overview_view(
            &system_report.system_info,
            &system_report.disks_report_info,
        );
        let bytes = web::Bytes::from(view);
        let body = once(ok::<_, Error>(bytes));

        HttpResponse::Ok().content_type("text/html").streaming(body)
    }

    #[get("/http-views/pages/cpu")]
    pub async fn render_cpu_as_html(app_state: web::Data<AppState>) -> impl Responder {
        let mut components = app_state.components.lock().unwrap();
        let mut disks = app_state.disks.lock().unwrap();
        let mut networks = app_state.networks.lock().unwrap();
        let mut system = app_state.system.lock().unwrap();

        let mut system_reporter =
            SystemReporter::new(&mut components, &mut disks, &mut networks, &mut system);
        let system_report = system_reporter.build_report();

        let view =
            HttpViews::get_cpu_view(&system_report.system_info, &system_report.cpu_report_info);
        let bytes = web::Bytes::from(view);
        let body = once(ok::<_, Error>(bytes));

        HttpResponse::Ok().content_type("text/html").streaming(body)
    }

    #[get("/http-views/pages/memory")]
    pub async fn render_memory_as_html(app_state: web::Data<AppState>) -> impl Responder {
        let mut components = app_state.components.lock().unwrap();
        let mut disks = app_state.disks.lock().unwrap();
        let mut networks = app_state.networks.lock().unwrap();
        let mut system = app_state.system.lock().unwrap();

        let mut system_reporter =
            SystemReporter::new(&mut components, &mut disks, &mut networks, &mut system);
        let system_report = system_reporter.build_report();

        let view = HttpViews::get_memory_view(&system_report.system_info);
        let bytes = web::Bytes::from(view);
        let body = once(ok::<_, Error>(bytes));
        HttpResponse::Ok().content_type("text/html").streaming(body)
    }

    #[get("/http-views/pages/disks")]
    pub async fn render_disks_as_html(app_state: web::Data<AppState>) -> impl Responder {
        let mut components = app_state.components.lock().unwrap();
        let mut disks = app_state.disks.lock().unwrap();
        let mut networks = app_state.networks.lock().unwrap();
        let mut system = app_state.system.lock().unwrap();

        let mut system_reporter =
            SystemReporter::new(&mut components, &mut disks, &mut networks, &mut system);
        let system_report = system_reporter.build_report();

        let view = HttpViews::get_disks_view(&system_report.disks_report_info);
        let bytes = web::Bytes::from(view);
        let body = once(ok::<_, Error>(bytes));
        HttpResponse::Ok().content_type("text/html").streaming(body)
    }

    #[get("/http-views/pages/components")]
    pub async fn render_components_as_html(app_state: web::Data<AppState>) -> impl Responder {
        let mut components = app_state.components.lock().unwrap();
        let mut disks = app_state.disks.lock().unwrap();
        let mut networks = app_state.networks.lock().unwrap();
        let mut system = app_state.system.lock().unwrap();

        let mut system_reporter =
            SystemReporter::new(&mut components, &mut disks, &mut networks, &mut system);
        let system_report = system_reporter.build_report();

        let view = HttpViews::get_components_view(&system_report.components_report_info);
        let bytes = web::Bytes::from(view);
        let body = once(ok::<_, Error>(bytes));
        HttpResponse::Ok().content_type("text/html").streaming(body)
    }

    #[get("/http-views/pages/networks")]
    pub async fn render_networks_as_html(app_state: web::Data<AppState>) -> impl Responder {
        let mut components = app_state.components.lock().unwrap();
        let mut disks = app_state.disks.lock().unwrap();
        let mut networks = app_state.networks.lock().unwrap();
        let mut system = app_state.system.lock().unwrap();

        let mut system_reporter =
            SystemReporter::new(&mut components, &mut disks, &mut networks, &mut system);
        let system_report = system_reporter.build_report();

        let view = HttpViews::get_network_view(&system_report.network_report_info);
        let bytes = web::Bytes::from(view);
        let body = once(ok::<_, Error>(bytes));
        HttpResponse::Ok().content_type("text/html").streaming(body)
    }
}
