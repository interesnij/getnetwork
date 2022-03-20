
extern crate diesel;

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use tera::{Tera, Context};
use actix_multipart::Multipart;
use std::borrow::BorrowMut;
use diesel::prelude::*;
use crate::utils::{
    split_payload,
    category_split_payload,
    get_template_2,
    establish_connection
};
use crate::schema;
use crate::models::{
    WikiCategories,
    NewWikiCategories,
    Wiki,
    NewWiki,
    WikiCategory,
    NewWikiCategory,
    WikiImage,
    NewWikiImage,
    WikiVideo,
    NewWikiVideo,
    TagItems,
    NewTagItems,
    Tag,
};

fn get_cats_for_wiki(wiki: &Wiki) -> Vec<WikiCategories> {
    use diesel::pg::expression::dsl::any;
    let _connection = establish_connection();

    let ids = WikiCategory::belonging_to(wiki).select(schema::wiki_category::wiki_categories_id);
    schema::wiki_categories::table
        .filter(schema::wiki_categories::id.eq(any(ids)))
        .load::<WikiCategories>(&_connection)
        .expect("could not load tags")
}
fn get_tags_for_wiki(wiki: &Wiki) -> Vec<Tag> {
    use crate::schema::tags_items::dsl::tags_items;
    use diesel::dsl::any;
    let _connection = establish_connection();

    let _tag_items = tags_items.filter(schema::tags_items::wiki_id.eq(&wiki.id)).load::<TagItems>(&_connection).expect("E");
    let mut stack = Vec::new();
    for _tag_item in _tag_items.iter() {
        stack.push(_tag_item.tag_id);
    };
    schema::tags::table
        .filter(schema::tags::id.eq(any(stack)))
        .load::<Tag>(&_connection)
        .expect("could not load tags")
}
fn get_6_wiki_for_category(category: &WikiCategories) -> Vec<Wiki> {
    use diesel::pg::expression::dsl::any;
    let _connection = establish_connection();

    let ids = WikiCategory::belonging_to(category).select(schema::wiki_category::wiki_id);
    schema::wikis::table
        .filter(schema::wikis::id.eq(any(ids)))
        .order(schema::wikis::wiki_created.desc())
        .limit(6)
        .load::<Wiki>(&_connection)
        .expect("could not load tags")
}
fn get_wiki_for_category(category: &WikiCategories) -> Vec<Wiki> {
    use diesel::pg::expression::dsl::any;
    let _connection = establish_connection();

    let ids = WikiCategory::belonging_to(category).select(schema::wiki_category::wiki_id);
    schema::wikis::table
        .filter(schema::wikis::id.eq(any(ids)))
        .order(schema::wikis::wiki_created.desc())
        .load::<Wiki>(&_connection)
        .expect("could not load tags")
}

pub async fn create_wiki_categories_page(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    let _template = _type + &"wikis/create_categories.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn create_wiki_page(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    use schema::tags::dsl::tags;

    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);

    let _connection = establish_connection();
    let all_tags :Vec<Tag> = tags
        .load(&_connection)
        .expect("Error.");

    data.insert("tags", &all_tags);
    let _template = _type + &"wikis/create_wiki.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn create_wiki_categories(mut payload: Multipart) -> impl Responder {
    use schema::wiki_categories;

    let _connection = establish_connection();
    let form = category_split_payload(payload.borrow_mut()).await;
    let new_cat = NewWikiCategories {
        name: form.name.clone(),
        wiki_position: form.position.clone(),
        image: Some(form.image.clone()),
        wiki_count: 0
    };
    let _new_wiki = diesel::insert_into(wiki_categories::table)
        .values(&new_cat)
        .get_result::<WikiCategories>(&_connection)
        .expect("Error saving post.");
    return HttpResponse::Ok();
}
pub async fn create_wiki(mut payload: Multipart) -> impl Responder {
    use schema::{wikis,wiki_images,wiki_videos,wiki_category,tags_items};
    use crate::schema::tags::dsl::tags;
    use crate::schema::wiki_categories::dsl::wiki_categories;

    let _connection = establish_connection();

    let form = split_payload(payload.borrow_mut()).await;
    let new_wiki = NewWiki::from_wiki_form(
        form.title.clone(),
        form.description.clone(),
        form.link.clone(),
        form.main_image.clone(),
        form.is_active.clone(),
        1
    );

    let _wiki = diesel::insert_into(wikis::table)
        .values(&new_wiki)
        .get_result::<Wiki>(&_connection)
        .expect("Error saving wiki.");

    for image in form.images.iter().enumerate() {
        let new_image = NewWikiImage::from_wiki_images_form(
            _wiki.id,
            image.1.to_string()
        );
        diesel::insert_into(wiki_images::table)
            .values(&new_image)
            .get_result::<WikiImage>(&_connection)
            .expect("Error saving wiki.");
        };
    for video in form.videos.iter().enumerate() {
        let new_video = NewWikiVideo::from_wiki_videos_form(
            _wiki.id,
            video.1.to_string()
        );
        diesel::insert_into(wiki_videos::table)
            .values(&new_video)
            .get_result::<WikiVideo>(&_connection)
            .expect("Error saving wiki.");
    };
    for category_id in form.category_list.iter().enumerate() {
        let new_category = NewWikiCategory {
            wiki_categories_id: *category_id.1,
            wiki_id: _wiki.id
        };
        diesel::insert_into(wiki_category::table)
            .values(&new_category)
            .get_result::<WikiCategory>(&_connection)
            .expect("Error saving wiki.");
        let _category = wiki_categories.filter(schema::wiki_categories::id.eq(category_id.1)).load::<WikiCategories>(&_connection).expect("E");
        diesel::update(&_category[0])
            .set(schema::wiki_categories::wiki_count.eq(_category[0].wiki_count + 1))
            .get_result::<WikiCategories>(&_connection)
            .expect("Error.");
    };
    for tag_id in form.tags_list.iter().enumerate() {
        let new_tag = NewTagItems{
            tag_id: *tag_id.1,
            service_id: 0,
            store_id: 0,
            blog_id: 0,
            wiki_id: _wiki.id,
            work_id: 0,
            tag_created: chrono::Local::now().naive_utc(),
        };
        diesel::insert_into(tags_items::table)
            .values(&new_tag)
            .get_result::<TagItems>(&_connection)
            .expect("Error.");
        let _tag = tags.filter(schema::tags::id.eq(tag_id.1)).load::<Tag>(&_connection).expect("E");
        diesel::update(&_tag[0])
            .set((schema::tags::tag_count.eq(_tag[0].tag_count + 1), schema::tags::wiki_count.eq(_tag[0].wiki_count + 1)))
            .get_result::<Tag>(&_connection)
            .expect("Error.");
    };
    HttpResponse::Ok()
}

pub async fn get_wiki_page(req: HttpRequest, tera: web::Data<Tera>, param: web::Path<(i32,i32)>) -> impl Responder {
    use schema::wikis::dsl::wikis;
    use schema::wiki_images::dsl::wiki_images;
    use schema::wiki_videos::dsl::wiki_videos;
    use schema::wiki_categories::dsl::wiki_categories;

    let _connection = establish_connection();
    let _wiki_id : i32 = param.1;
    let _cat_id : i32 = param.0;

    let _wiki = wikis.filter(schema::wikis::id.eq(&_wiki_id)).load::<Wiki>(&_connection).expect("E");

    let _s_category = wiki_categories
        .filter(schema::wiki_categories::id.eq(&_cat_id))
        .load::<WikiCategories>(&_connection)
        .expect("E");

    let mut data = Context::new();

    let _category_wikis = get_wiki_for_category(&_s_category[0]);
    let _category_wikis_len : usize = _category_wikis.len();
    for (i, item) in _category_wikis.iter().enumerate().rev() {
        if item.id == _wiki_id {
            if (i + 1) != _category_wikis_len {
                let _prev = Some(&_category_wikis[i + 1]);
                data.insert("prev", &_prev);
            };
            if i != 0 {
                let _next = Some(&_category_wikis[i - 1]);
                data.insert("next", &_next);
            };
            break;
        }
    };

    let _images :Vec<WikiImage> = wiki_images.filter(schema::wiki_images::wiki.eq(&_wiki_id)).load(&_connection).expect("E");
    let _videos :Vec<WikiVideo> = wiki_videos.filter(schema::wiki_videos::wiki.eq(&_wiki_id)).load(&_connection).expect("E");
    let _categories = get_cats_for_wiki(&_wiki[0]);
    let _tags = get_tags_for_wiki(&_wiki[0]);

    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("object", &_wiki[0]);
    data.insert("images", &_images);
    data.insert("videos", &_videos);
    data.insert("categories", &_categories);
    data.insert("category", &_s_category[0]);
    data.insert("tags", &_tags);
    data.insert("tags_count", &_tags.len());
    data.insert("is_admin", &_is_admin);

    let _template = _type + &"wikis/wiki.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn wiki_category_page(req: HttpRequest, tera: web::Data<Tera>, id: web::Path<i32>) -> impl Responder {
    use schema::wiki_categories::dsl::wiki_categories;
    use diesel::dsl::any;
    use crate::schema::tags_items::dsl::tags_items;

    let mut data = Context::new();
    let page_size = 20;
    let mut offset = 0;

    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    let _connection = establish_connection();

    let _category = wiki_categories.filter(schema::wiki_categories::id.eq(*id)).load::<WikiCategories>(&_connection).expect("E");

    data.insert("category", &_category[0]);

    loop {
        let ids = WikiCategory::belonging_to(&_category).select(schema::wiki_category::wiki_id);
        let _wikis = schema::wikis::table
        .filter(schema::wikis::id.eq(any(ids)))
        .limit(page_size)
        .offset(offset)
        .order(schema::wikis::wiki_created.desc())
        .load::<Wiki>(&_connection)
        .expect("could not load tags");
         if _wikis.len() <= 0 { break;}
         offset += page_size;
         data.insert("wikis", &_wikis);
    };

    let mut stack = Vec::new();
    let _tag_items = tags_items.filter(schema::tags_items::wiki_id.ne(0)).load::<TagItems>(&_connection).expect("E");
    for _tag_item in _tag_items.iter() {
        if stack.iter().any(|&i| i==_tag_item.tag_id) {
            continue;
        } else {
            stack.push(_tag_item.tag_id);
        }
    };
    let _tags = schema::tags::table
        .filter(schema::tags::id.eq(any(stack)))
        .load::<Tag>(&_connection)
        .expect("could not load tags");

    data.insert("tags", &_tags);
    data.insert("tags_count", &_tags.len());

    let _template = _type + &"wikis/category.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn wiki_categories_page(req: HttpRequest, tera: web::Data<Tera>) -> impl Responder {
    use diesel::dsl::any;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::wikis::dsl::wikis;

    let _connection = establish_connection();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    let mut data = Context::new();
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);

    let _wikis = wikis.filter(schema::wikis::is_wiki_active.eq(true)).load::<Wiki>(&_connection).expect("E");
    let mut _count: i32 = 0;
    for _cat in _wiki_cats.iter() {
        _count += 1;
        // для генерации переменной 1 2 3
        let mut _let_int : String = _count.to_string().parse().unwrap();
        let _let_data_wikis: String = "wikis".to_string() + &_let_int;
        data.insert(&_let_data_wikis, &get_6_wiki_for_category(_cat));
    };


    let mut stack = Vec::new();
    for wiki in _wikis.iter() {
        let _tag_items = tags_items.filter(schema::tags_items::wiki_id.eq(wiki.id)).load::<TagItems>(&_connection).expect("E");
        for _tag_item in _tag_items.iter() {
            if stack.iter().any(|&i| i==_tag_item.tag_id) {
                continue;
            } else {
                stack.push(_tag_item.tag_id);
            }
        };
    };
    let _tags = schema::tags::table
        .filter(schema::tags::id.eq(any(stack)))
        .load::<Tag>(&_connection)
        .expect("could not load tags");

    data.insert("tags", &_tags);
    data.insert("tags_count", &_tags.len());

    let _template = _type + &"wikis/categories.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn edit_wiki_page(req: HttpRequest, tera: web::Data<Tera>, _id: web::Path<i32>) -> impl Responder {
    use schema::wikis::dsl::*;
    use schema::tags::dsl::*;
    use crate::schema::wiki_images::dsl::wiki_images;
    use crate::schema::wiki_videos::dsl::wiki_videos;

    let _wiki_id : i32 = *_id;
    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    let _connection = establish_connection();
    let _wiki = wikis.filter(schema::wikis::id.eq(&_wiki_id)).load::<Wiki>(&_connection).expect("E");

    let _categories = get_cats_for_wiki(&_wiki[0]);
    let _all_tags :Vec<Tag> = tags.load(&_connection).expect("Error.");
    let _wiki_tags = get_tags_for_wiki(&_wiki[0]);

    let _images = wiki_images.filter(schema::wiki_images::wiki.eq(_wiki[0].id)).load::<WikiImage>(&_connection).expect("E");
    let _videos = wiki_videos.filter(schema::wiki_videos::wiki.eq(_wiki[0].id)).load::<WikiVideo>(&_connection).expect("E");

    data.insert("wiki", &_wiki[0]);
    data.insert("wiki_tags", &_wiki_tags);
    data.insert("all_tags", &_all_tags);
    data.insert("categories", &_categories);
    data.insert("images", &_images);
    data.insert("videos", &_videos);

    let _template = _type + &"wikis/edit_wiki.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct WikiParams {
    content: String,
}
pub async fn edit_content_wiki_page(req: HttpRequest, tera: web::Data<Tera>, _id: web::Path<i32>) -> impl Responder {
    use schema::wikis::dsl::*;

    let _wiki_id : i32 = *_id;
    let _connection = establish_connection();
    let _wiki = wikis.filter(schema::wikis::id.eq(&_wiki_id)).load::<Wiki>(&_connection).expect("E");

    let params = web::Query::<WikiParams>::from_query(&req.query_string()).unwrap();
    if params.content.clone() != "".to_string() {
        diesel::update(&_wiki[0])
            .set(schema::wikis::content.eq(&params.content.clone()))
            .get_result::<Wiki>(&_connection)
            .expect("E.");
    }

    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    data.insert("wiki", &_wiki[0]);

    let _template = _type + &"wikis/edit_content_wiki.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}

pub async fn edit_wiki_category_page(req: HttpRequest, tera: web::Data<Tera>, _id: web::Path<i32>) -> impl Responder {
    use schema::wiki_categories::dsl::*;

    let _cat_id : i32 = *_id;
    let mut data = Context::new();
    let (_type, _is_admin, _service_cats, _store_cats, _blog_cats, _wiki_cats, _work_cats) = get_template_2(req);
    data.insert("service_categories", &_service_cats);
    data.insert("store_categories", &_store_cats);
    data.insert("blog_categories", &_blog_cats);
    data.insert("wiki_categories", &_wiki_cats);
    data.insert("work_categories", &_work_cats);
    data.insert("is_admin", &_is_admin);
    let _connection = establish_connection();
    let _category = wiki_categories.filter(schema::wiki_categories::id.eq(&_cat_id)).load::<WikiCategories>(&_connection).expect("E");

    data.insert("category", &_category[0]);
    let _template = _type + &"wikis/edit_category.html".to_string();
    let _rendered = tera.render(&_template, &data).unwrap();
    HttpResponse::Ok().body(_rendered)
}


pub async fn edit_wiki(mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::models::EditWiki;
    use crate::schema::wikis::dsl::wikis;
    use crate::schema::wiki_category::dsl::wiki_category;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::wiki_videos::dsl::wiki_videos;
    use crate::schema::wiki_images::dsl::wiki_images;
    use crate::schema::wiki_categories::dsl::wiki_categories;
    use crate::schema::tags::dsl::tags;

    let _connection = establish_connection();
    let _wiki_id : i32 = *_id;
    let _wiki = wikis.filter(schema::wikis::id.eq(_wiki_id)).load::<Wiki>(&_connection).expect("E");

    let _categories = get_cats_for_wiki(&_wiki[0]);
    let _tags = get_tags_for_wiki(&_wiki[0]);
    for _category in _categories.iter() {
        diesel::update(_category)
            .set(schema::wiki_categories::wiki_count.eq(_category.wiki_count - 1))
            .get_result::<WikiCategories>(&_connection)
            .expect("Error.");
    };
    for _tag in _tags.iter() {
        diesel::update(_tag)
            .set((schema::tags::tag_count.eq(_tag.tag_count - 1), schema::tags::wiki_count.eq(_tag.wiki_count - 1)))
            .get_result::<Tag>(&_connection)
            .expect("Error.");
    };

    diesel::delete(wiki_images.filter(schema::wiki_images::wiki.eq(_wiki_id))).execute(&_connection).expect("E");
    diesel::delete(wiki_videos.filter(schema::wiki_videos::wiki.eq(_wiki_id))).execute(&_connection).expect("E");
    diesel::delete(tags_items.filter(schema::tags_items::wiki_id.eq(_wiki_id))).execute(&_connection).expect("E");
    diesel::delete(wiki_category.filter(schema::wiki_category::wiki_id.eq(_wiki_id))).execute(&_connection).expect("E");

    let form = split_payload(payload.borrow_mut()).await;
    let _new_wiki = EditWiki {
        title: form.title.clone(),
        description: Some(form.description.clone()),
        link: Some(form.link.clone()),
        image: Some(form.main_image.clone()),
        is_wiki_active: form.is_active.clone()
    };

    diesel::update(&_wiki[0])
        .set(_new_wiki)
        .get_result::<Wiki>(&_connection)
        .expect("E");

    for _image in form.images.iter().enumerate() {
        let new_edit_image = NewWikiImage::from_wiki_images_form(
            _wiki_id,
            _image.1.to_string()
        );
        diesel::insert_into(schema::wiki_images::table)
            .values(&new_edit_image)
            .get_result::<WikiImage>(&_connection)
            .expect("E.");
        };
    for _video in form.videos.iter().enumerate() {
        let new_video = NewWikiVideo::from_wiki_videos_form(
            _wiki_id,
            _video.1.to_string()
        );
        diesel::insert_into(schema::wiki_videos::table)
            .values(&new_video)
            .get_result::<WikiVideo>(&_connection)
            .expect("E.");
    };
    for category_id in form.category_list.iter().enumerate() {
        let new_category = NewWikiCategory {
            wiki_categories_id: *category_id.1,
            wiki_id: _wiki_id
        };
        diesel::insert_into(schema::wiki_category::table)
            .values(&new_category)
            .get_result::<WikiCategory>(&_connection)
            .expect("E.");
        let _category_2 = wiki_categories.filter(schema::wiki_categories::id.eq(category_id.1)).load::<WikiCategories>(&_connection).expect("E");
        diesel::update(&_category_2[0])
            .set(schema::wiki_categories::wiki_count.eq(_category_2[0].wiki_count + 1))
            .get_result::<WikiCategories>(&_connection)
            .expect("Error.");
    };
    for _tag_id in form.tags_list.iter().enumerate() {
        let _new_tag = NewTagItems{
            tag_id: *_tag_id.1,
            service_id: 0,
            store_id: 0,
            blog_id: 0,
            wiki_id: _wiki_id,
            work_id: 0,
            tag_created: chrono::Local::now().naive_utc(),
        };
        diesel::insert_into(schema::tags_items::table)
            .values(&_new_tag)
            .get_result::<TagItems>(&_connection)
            .expect("Error.");
        let _tag_2 = tags.filter(schema::tags::id.eq(_tag_id.1)).load::<Tag>(&_connection).expect("E");
        diesel::update(&_tag_2[0])
            .set((schema::tags::tag_count.eq(_tag_2[0].tag_count + 1), schema::tags::wiki_count.eq(_tag_2[0].wiki_count + 1)))
            .get_result::<Tag>(&_connection)
            .expect("Error.");
    };
    HttpResponse::Ok()
}

pub async fn edit_wiki_category(mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    use crate::models::EditWikiCategories;
    use crate::schema::wiki_categories::dsl::wiki_categories;

    let _connection = establish_connection();
    let _cat_id : i32 = *_id;
    let _category = wiki_categories.filter(schema::wiki_categories::id.eq(_cat_id)).load::<WikiCategories>(&_connection).expect("E");

    let form = category_split_payload(payload.borrow_mut()).await;
    let _new_cat = EditWikiCategories {
        name: form.name.clone(),
        wiki_position: form.position.clone(),
        image: Some(form.image.clone()),
        wiki_count: _category[0].wiki_count,
    };

    diesel::update(&_category[0])
        .set(_new_cat)
        .get_result::<WikiCategories>(&_connection)
        .expect("E");
    HttpResponse::Ok()
}


pub async fn delete_wiki(_id: web::Path<i32>) -> impl Responder {
    use crate::schema::wikis::dsl::wikis;
    use crate::schema::wiki_category::dsl::wiki_category;
    use crate::schema::tags_items::dsl::tags_items;
    use crate::schema::wiki_videos::dsl::wiki_videos;
    use crate::schema::wiki_images::dsl::wiki_images;

    let _connection = establish_connection();
    let _wiki_id : i32 = *_id;
    let _wiki = wikis.filter(schema::wikis::id.eq(_wiki_id)).load::<Wiki>(&_connection).expect("E");

    let _categories = get_cats_for_wiki(&_wiki[0]);
    let _tags = get_tags_for_wiki(&_wiki[0]);
    for _category in _categories.iter() {
        diesel::update(_category)
            .set(schema::wiki_categories::wiki_count.eq(_category.wiki_count - 1))
            .get_result::<WikiCategories>(&_connection)
            .expect("Error.");
    };
    for _tag in _tags.iter() {
        diesel::update(_tag)
            .set((schema::tags::tag_count.eq(_tag.tag_count - 1), schema::tags::wiki_count.eq(_tag.wiki_count - 1)))
            .get_result::<Tag>(&_connection)
            .expect("Error.");
    };

    diesel::delete(wiki_images.filter(schema::wiki_images::wiki.eq(_wiki_id))).execute(&_connection).expect("E");
    diesel::delete(wiki_videos.filter(schema::wiki_videos::wiki.eq(_wiki_id))).execute(&_connection).expect("E");
    diesel::delete(tags_items.filter(schema::tags_items::wiki_id.eq(_wiki_id))).execute(&_connection).expect("E");
    diesel::delete(wiki_category.filter(schema::wiki_category::wiki_id.eq(_wiki_id))).execute(&_connection).expect("E");
    diesel::delete(&_wiki[0]).execute(&_connection).expect("E");
    HttpResponse::Ok()
}
pub async fn delete_wiki_category(_id: web::Path<i32>) -> impl Responder {
    use crate::schema::wiki_categories::dsl::wiki_categories;

    let _connection = establish_connection();
    let _cat_id : i32 = *_id;
    let _category = wiki_categories.filter(schema::wiki_categories::id.eq(_cat_id)).load::<WikiCategories>(&_connection).expect("E");
    diesel::delete(wiki_categories.filter(schema::wiki_categories::id.eq(_cat_id))).execute(&_connection).expect("E");
    HttpResponse::Ok()
}
