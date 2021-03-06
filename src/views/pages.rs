use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    error::InternalError,
    http::StatusCode,
};
use crate::models::User;
use serde::Deserialize;
use crate::utils::{
    establish_connection,
    get_device_and_ajax,
    get_request_user_data,
    is_signed_in,
};
use crate::diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use actix_session::Session;
use sailfish::TemplateOnce;


pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index));
    config.route("/about/", web::get().to(about));
    //config.route("/feedback/", web::post().to(create_feedback));
    //config.route("/feedback_list/", web::get().to(feedback_list_page));
    //config.route("/serve_list/", web::get().to(serve_list_page));
    //config.route("/load_item/", web::get().to(get_load_page));
}

#[derive(Debug, Deserialize)]
pub struct SParams {
    pub q: String,
}
pub async fn index(req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
    use crate::models::{Work, Service, Wiki, Blog, Store};

    let _connection = establish_connection();
    let _last_works = Work::get_3_works();
    let _last_services = Service::get_6_services();
    let _last_wikis = Wiki::get_3_wikis();
    let _last_blogs = Blog::get_3_blogs();
    let _last_stores = Store::get_3_stores();
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/mainpage.stpl")]
            struct Template {
                request_user:  User,
                last_works:    Vec<Work>,
                last_services: Vec<Service>,
                last_wikis:    Vec<Wiki>,
                last_blogs:    Vec<Blog>,
                last_stores:   Vec<Store>,
                is_ajax:       bool,
            }
            let body = Template {
                request_user:  _request_user,
                last_works:    _last_works,
                last_services: _last_services,
                last_wikis:    _last_wikis,
                last_blogs:    _last_blogs,
                last_stores:   _last_stores,
                is_ajax:       is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/mainpage.stpl")]
            struct Template {
                request_user:  User,
                last_works:    Vec<Work>,
                last_services: Vec<Service>,
                last_wikis:    Vec<Wiki>,
                last_blogs:    Vec<Blog>,
                last_stores:   Vec<Store>,
                is_ajax:       bool,
            }
            let body = Template {
                request_user:  _request_user,
                last_works:    _last_works,
                last_services: _last_services,
                last_wikis:    _last_wikis,
                last_blogs:    _last_blogs,
                last_stores:   _last_stores,
                is_ajax:       is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/anon_mainpage.stpl")]
            struct Template {
                last_works:    Vec<Work>,
                last_services: Vec<Service>,
                last_wikis:    Vec<Wiki>,
                last_blogs:    Vec<Blog>,
                last_stores:   Vec<Store>,
                is_ajax:       bool,
            }
            let body = Template {
                last_works:    _last_works,
                last_services: _last_services,
                last_wikis:    _last_wikis,
                last_blogs:    _last_blogs,
                last_stores:   _last_stores,
                is_ajax:       is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/anon_mainpage.stpl")]
            struct Template {
                last_works:    Vec<Work>,
                last_services: Vec<Service>,
                last_wikis:    Vec<Wiki>,
                last_blogs:    Vec<Blog>,
                last_stores:   Vec<Store>,
                is_ajax:       bool,
            }
            let body = Template {
                last_works:    _last_works,
                last_services: _last_services,
                last_wikis:    _last_wikis,
                last_blogs:    _last_blogs,
                last_stores:   _last_stores,
                is_ajax:       is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn about(req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = get_device_and_ajax(&req);

    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/pages/about.stpl")]
            struct Template {
                request_user:  User,
                is_ajax:       bool,
            }
            let body = Template {
                request_user:  _request_user,
                is_ajax:       is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/pages/about.stpl")]
            struct Template {
                request_user:  User,
                is_ajax:       bool,
            }
            let body = Template {
                request_user:  _request_user,
                is_ajax:       is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/pages/anon_about.stpl")]
            struct Template {
                is_ajax:       bool,
            }
            let body = Template {
                is_ajax:       is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/pages/anon_about.stpl")]
            struct Template {
                is_ajax:       bool,
            }
            let body = Template {
                is_ajax:       is_ajax,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

pub async fn create_feedback(mut payload: actix_multipart::Multipart) -> impl Responder {
    use crate::schema::feedbacks;
    use std::borrow::BorrowMut;
    use crate::models::{Feedback, NewFeedback};
    use crate::utils::feedback_form;

    let _connection = establish_connection();
    let form = feedback_form(payload.borrow_mut()).await;
    let new_feedback = NewFeedback {
        username: form.username.clone(),
        email:    form.email.clone(),
        message:  form.message.clone()
    };
    let _new_feedback = diesel::insert_into(feedbacks::table)
        .values(&new_feedback)
        .get_result::<Feedback>(&_connection)
        .expect("E.");
    return HttpResponse::Ok();
}

pub async fn feedback_list_page(req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
        if !is_signed_in(&session) {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied"))
        }
        else {
            use crate::schema::feedbacks::dsl::feedbacks;
            use crate::models::Feedback;

            let _connection = establish_connection();
            let _feedbacks = feedbacks
                .load::<Feedback>(&_connection)
                .expect("E");

            let _request_user = get_request_user_data(&session);
            let (is_desctop, is_ajax) = get_device_and_ajax(&req);
            if _request_user.perm < 60 {
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Permission Denied"))
            }
            else if is_desctop {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/main/feedback_list.stpl")]
                struct Template {
                    is_ajax:       bool,
                    feedback_list: Vec<Feedback>
                }
                let body = Template {
                    is_ajax:       is_ajax,
                    feedback_list: _feedbacks,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
            else {
                #[derive(TemplateOnce)]
                #[template(path = "mobile/main/feedback_list.stpl")]
                struct Template {
                    is_ajax:       bool,
                    feedback_list: Vec<Feedback>
                }
                let body = Template {
                    is_ajax:       is_ajax,
                    feedback_list: _feedbacks,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
            }
        }
}

pub async fn serve_list_page(req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
    use crate::models::TechCategories;
    use crate::schema;
    use crate::schema::tech_categories::dsl::tech_categories;

    let _connection = establish_connection();
    let all_tech_categories: Vec<TechCategories> = tech_categories
        .order(schema::tech_categories::position.asc())
        .load(&_connection)
        .expect("E.");

    let (is_desctop, is_ajax) = get_device_and_ajax(&req);
    if is_signed_in(&session) {
        let _request_user = get_request_user_data(&session);
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/serve_list.stpl")]
            struct Template {
                request_user: User,
                is_ajax:      bool,
                t_categories: Vec<TechCategories>
            }
            let body = Template {
                request_user: _request_user,
                is_ajax:      is_ajax,
                t_categories: all_tech_categories,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/serve_list.stpl")]
            struct Template {
                request_user: User,
                is_ajax:      bool,
                t_categories: Vec<TechCategories>
            }
            let body = Template {
                request_user: _request_user,
                is_ajax:      is_ajax,
                t_categories: all_tech_categories,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        if is_desctop {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/anon_serve_list.stpl")]
            struct Template {
                is_ajax:      bool,
                t_categories: Vec<TechCategories>
            }
            let body = Template {
                is_ajax:      is_ajax,
                t_categories: all_tech_categories,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "mobile/main/anon_serve_list.stpl")]
            struct Template {
                is_ajax:      bool,
                t_categories: Vec<TechCategories>
            }
            let body = Template {
                is_ajax:      is_ajax,
                t_categories: all_tech_categories,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LoadParams {
    pub _object_type: String,
    pub _owner_type:  String,
    pub _object_pk:   i32,
    pub _owner_pk:    i32,
}
pub async fn get_load_page(req: HttpRequest, session: Session) -> actix_web::Result<HttpResponse> {
    use crate::schema;

    let mut _object_type: String = "".to_string();
    let mut _owner_type: String = "".to_string();
    let mut _object_pk: i32 = 0;
    let mut _owner_pk: i32 = 0;

    let _connection = establish_connection();
    let params = web::Query::<LoadParams>::from_query(&req.query_string());
    if params.is_ok() {
        let wrap = params.unwrap();
        if wrap._object_type != "".to_string() {
            _object_type = wrap._object_type.clone();
        }
        if wrap._owner_type != "".to_string() {
            _owner_type = wrap._owner_type.clone();
        }
        if wrap._object_pk != 0 {
            _object_pk = wrap._object_pk.clone();
        }
        if wrap._owner_pk != 0 {
            _owner_pk = wrap._owner_pk.clone();
        }
    }

    if _object_type == "serve_category".to_string() {
        use crate::models::ServeCategories;
        use crate::schema::serve_categories::dsl::serve_categories;

        let _serve_category = serve_categories
            .filter(schema::serve_categories::id.eq(&_object_pk))
            .load::<ServeCategories>(&_connection)
            .expect("E");

            #[derive(TemplateOnce)]
            #[template(path = "desctop/load/serve_category.stpl")]
            struct Template {
                object:      ServeCategories,
                object_type: String
            }
            let body = Template {
                object:      _serve_category.into_iter().nth(0).unwrap(),
                object_type: "serve_category".to_string(),
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))

    } else if _object_type == "serve".to_string() && _owner_type != "service".to_string() {
        use crate::models::Serve;
        use crate::schema::serve::dsl::serve;

        let _serve = serve
            .filter(schema::serve::id.eq(&_object_pk))
            .load::<Serve>(&_connection)
            .expect("E");

        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/serve.stpl")]
        struct Template {
            object:      Serve,
            object_type: String
        }
        let body = Template {
            object:      _serve.into_iter().nth(0).unwrap(),
            object_type: "serve".to_string(),
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else if _object_type == "serve".to_string() && _owner_type == "service".to_string() {
        use crate::models::{Serve, Service};
        use crate::schema::{
            serve::dsl::serve,
            services::dsl::services
        };

        let _serve = serve
            .filter(schema::serve::id.eq(&_object_pk))
            .load::<Serve>(&_connection)
            .expect("E");

        let _service_id: i32 = _owner_pk;
        let _service = services
            .filter(schema::services::id.eq(&_service_id))
            .load::<Service>(&_connection)
            .expect("E");

        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/serve.stpl")]
        struct Template {
            object:      Serve,
            object_type: String,
            service:     Service,
        }
        let body = Template {
            object:      _serve.into_iter().nth(0).unwrap(),
            object_type: "serve".to_string(),
            service:     _service.into_iter().nth(0).unwrap(),
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
}
