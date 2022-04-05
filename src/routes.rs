use actix_web::web;

use crate::views::{
    work_progs,
    blog_progs,
    service_progs,
    store_progs,
    wiki_progs,
    tag_progs,
    serve_progs,
    search_progs,
    pages,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    // pages urls
    .route("/", web::get().to(pages::index))
    .route("/about/", web::get().to(pages::about))
    .route("/signup", web::get().to(pages::signup))
    .route("/signup", web::post().to(pages::process_signup))
    .route("/feedback/", web::post().to(pages::create_feedback))
    .route("/feedback_list/", web::get().to(pages::feedback_list_page))
    .route("/serve_list/", web::get().to(pages::serve_list_page))
    .route("/load_item/", web::get().to(pages::get_load_page))

    .configure(blog_progs::blog_routes)
    // search urls
    .route("/search/", web::get().to(search_progs::search_page))
    .route("/search_blogs/", web::get().to(search_progs::search_blogs_page))
    .route("/search_services/", web::get().to(search_progs::search_services_page))
    .route("/search_stores/", web::get().to(search_progs::search_stores_page))
    .route("/search_wikis/", web::get().to(search_progs::search_wikis_page))
    .route("/search_works/", web::get().to(search_progs::search_works_page))

    // serve urls
    .route("/serve/{id}/", web::get().to(serve_progs::get_serve_page))
    .route("/serve_categories/", web::get().to(serve_progs::serve_categories_page))

    .service(web::resource("/create_tech_categories/")
        .route(web::get().to(serve_progs::create_tech_categories_page))
        .route(web::post().to(serve_progs::create_tech_categories))
    )
    .service(web::resource("/create_serve_categories/")
        .route(web::get().to(serve_progs::create_serve_categories_page))
        .route(web::post().to(serve_progs::create_serve_categories))
    )
    .service(web::resource("/edit_tech_category/{id}/")
        .route(web::get().to(serve_progs::edit_tech_category_page))
        .route(web::post().to(serve_progs::edit_tech_category))
    )
    .service(web::resource("/edit_serve_category/{id}/")
        .route(web::get().to(serve_progs::edit_serve_category_page))
        .route(web::post().to(serve_progs::edit_serve_category))
    )

    .service(web::resource("/create_serve/")
        .route(web::get().to(serve_progs::create_serve_page))
        .route(web::post().to(serve_progs::create_serve))
    )
    .service(web::resource("/edit_serve/{id}/")
        .route(web::get().to(serve_progs::edit_serve_page))
        .route(web::post().to(serve_progs::edit_serve))
    )
    .route("/delete_serve/{id}/", web::get().to(serve_progs::delete_serve))
    .route("/delete_serve_category/{id}/", web::get().to(serve_progs::delete_serve_category))
    .route("/delete_tech_category/{id}/", web::get().to(serve_progs::delete_tech_category))

    // tags urls
    .route("/tags/", web::get().to(tag_progs::tags_page))
    .route("/tag/{id}/", web::get().to(tag_progs::tag_page))
    .route("/tag_blogs/{id}/", web::get().to(tag_progs::tag_blogs_page))
    .route("/tag_services/{id}/", web::get().to(tag_progs::tag_services_page))
    .route("/tag_stores/{id}/", web::get().to(tag_progs::tag_stores_page))
    .route("/tag_wikis/{id}/", web::get().to(tag_progs::tag_wikis_page))
    .route("/tag_works/{id}/", web::get().to(tag_progs::tag_works_page))
    .service(web::resource("/create_tag/")
        .route(web::get().to(tag_progs::create_tag_page))
        .route(web::post().to(tag_progs::create_tag))
    )
    .service(web::resource("/edit_tag/{id}/")
        .route(web::get().to(tag_progs::edit_tag_page))
        .route(web::post().to(tag_progs::edit_tag))
    )
    .route("/delete_tag/{id}/", web::get().to(tag_progs::delete_tag))

    // portfolio urls
    .route("/work_categories/", web::get().to(work_progs::work_categories_page))
    .service(web::resource("/create_work_categories/")
        .route(web::get().to(work_progs::create_work_categories_page))
        .route(web::post().to(work_progs::create_work_categories))
    )
    .service(web::resource("/edit_work_category/{id}/")
        .route(web::get().to(work_progs::edit_work_category_page))
        .route(web::post().to(work_progs::edit_work_category))
    )
    .service(web::resource("/create_work/")
        .route(web::get().to(work_progs::create_work_page))
        .route(web::post().to(work_progs::create_work))
    )
    .service(web::resource("/edit_work/{id}/")
        .route(web::get().to(work_progs::edit_work_page))
        .route(web::post().to(work_progs::edit_work))
    )
    .route("/edit_content_work/{id}/", web::get().to(work_progs::edit_content_work_page))
    .route("/delete_work/{id}/", web::get().to(work_progs::delete_work))
    .route("/delete_work_category/{id}/", web::get().to(work_progs::delete_work_category))
    .service(web::resource("/work/{cat_id}/{work_id}/").route(web::get().to(work_progs::get_work_page)))
    .service(web::resource("/work/{id}/").route(web::get().to(work_progs::work_category_page)))

    // store urls
    .route("/store_categories/", web::get().to(store_progs::store_categories_page))
    .service(web::resource("/create_store_categories/")
        .route(web::get().to(store_progs::create_store_categories_page))
        .route(web::post().to(store_progs::create_store_categories))
    )
    .service(web::resource("/edit_store_category/{id}/")
        .route(web::get().to(store_progs::edit_store_category_page))
        .route(web::post().to(store_progs::edit_store_category))
    )
    .service(web::resource("/create_store/")
        .route(web::get().to(store_progs::create_store_page))
        .route(web::post().to(store_progs::create_store))
    )
    .service(web::resource("/edit_store/{id}/")
        .route(web::get().to(store_progs::edit_store_page))
        .route(web::post().to(store_progs::edit_store))
    )
    .route("/edit_content_store/{id}/", web::get().to(store_progs::edit_content_store_page))
    .route("/delete_store/{id}/", web::get().to(store_progs::delete_store))
    .route("/delete_store_category/{id}/", web::get().to(store_progs::delete_store_category))
    .service(web::resource("/store/{cat_id}/{store_id}/").route(web::get().to(store_progs::get_store_page)))
    .service(web::resource("/store/{id}/").route(web::get().to(store_progs::store_category_page)))

    // service urls
    .route("/service_categories/", web::get().to(service_progs::service_categories_page))
    .service(web::resource("/create_service_categories/")
        .route(web::get().to(service_progs::create_service_categories_page))
        .route(web::post().to(service_progs::create_service_categories))
    )
    .service(web::resource("/edit_service_category/{id}/")
        .route(web::get().to(service_progs::edit_service_category_page))
        .route(web::post().to(service_progs::edit_service_category))
    )
    .service(web::resource("/create_service/")
        .route(web::get().to(service_progs::create_service_page))
        .route(web::post().to(service_progs::create_service))
    )
    .service(web::resource("/edit_service/{id}/")
        .route(web::get().to(service_progs::edit_service_page))
        .route(web::post().to(service_progs::edit_service))
    )
    .route("/edit_content_service/{id}/", web::get().to(service_progs::edit_content_service_page))
    .route("/delete_service/{id}/", web::get().to(service_progs::delete_service))
    .route("/delete_service_category/{id}/", web::get().to(service_progs::delete_service_category))
    .service(web::resource("/service/{cat_id}/{service_id}/").route(web::get().to(service_progs::get_service_page)))
    .service(web::resource("/service/{id}/").route(web::get().to(service_progs::service_category_page)))

    // wiki urls
    .route("/wiki_categories/", web::get().to(wiki_progs::wiki_categories_page))
    .service(web::resource("/create_wiki_categories/")
        .route(web::get().to(wiki_progs::create_wiki_categories_page))
        .route(web::post().to(wiki_progs::create_wiki_categories))
    )
    .service(web::resource("/edit_wiki_category/{id}/")
        .route(web::get().to(wiki_progs::edit_wiki_category_page))
        .route(web::post().to(wiki_progs::edit_wiki_category))
    )
    .service(web::resource("/create_wiki/")
        .route(web::get().to(wiki_progs::create_wiki_page))
        .route(web::post().to(wiki_progs::create_wiki))
    )
    .service(web::resource("/edit_wiki/{id}/")
        .route(web::get().to(wiki_progs::edit_wiki_page))
        .route(web::post().to(wiki_progs::edit_wiki))
    )
    .route("/edit_content_wiki/{id}/", web::get().to(wiki_progs::edit_content_wiki_page))
    .route("/delete_wiki/{id}/", web::get().to(wiki_progs::delete_wiki))
    .route("/delete_wiki_category/{id}/", web::get().to(wiki_progs::delete_wiki_category))
    .service(web::resource("/wiki/{cat_id}/{wiki_id}/").route(web::get().to(wiki_progs::get_wiki_page)))
    .service(web::resource("/wiki/{id}/").route(web::get().to(wiki_progs::wiki_category_page)))
    ;
}
