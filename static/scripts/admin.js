function setEndOfContenteditable(contentEditableElement) {
    var range,selection;
    if(document.createRange) {
        range = document.createRange();
        range.selectNodeContents(contentEditableElement);
        range.collapse(false);
        selection = window.getSelection();
        selection.removeAllRanges();
        selection.addRange(range);
    }
    else if(document.selection) {
        range = document.body.createTextRange();
        range.moveToElementText(contentEditableElement);
        range.collapse(false);
        range.select();
    }
};

function format_text(text) {
  text.innerHTML = text.innerHTML.replace(/&nbsp;/g, ' ');
  br = text.querySelectorAll("br");
  text.querySelectorAll("br");
  img = text.querySelectorAll("img");
  p = text.querySelectorAll("p");
  ol = text.querySelectorAll("ol");
  ul = text.querySelectorAll("ul");
  a = text.querySelectorAll("a");
  h1 = text.querySelectorAll("h1");
  h2 = text.querySelectorAll("h2");
  h3 = text.querySelectorAll("h3");
  h4 = text.querySelectorAll("h4");
  h5 = text.querySelectorAll("h5");
  h6 = text.querySelectorAll("h6");
  div = text.querySelectorAll("div");
  span = text.querySelectorAll("span");

  for (var i = 0; i < br.length; i++){
      br[i].removeAttribute("style"); br[i].removeAttribute("class")
  };
  for (var i = 0; i < img.length; i++){
      img[i].removeAttribute("style"); img[i].removeAttribute("class")
  };
  for (var i = 0; i < p.length; i++){
      p[i].removeAttribute("style"); p[i].removeAttribute("class")
  };
  for (var i = 0; i < ul.length; i++){
      ul[i].removeAttribute("style"); ul[i].removeAttribute("class")
  };
  for (var i = 0; i < ol.length; i++){
      ol[i].removeAttribute("style"); ol[i].removeAttribute("class")
  };
  for (var i = 0; i < a.length; i++){
      a[i].removeAttribute("style"); a[i].removeAttribute("class")
  };
  for (var i = 0; i < span.length; i++){
      span[i].removeAttribute("style"); span[i].removeAttribute("class")
  };
  for (var i = 0; i < h1.length; i++){
      h1[i].removeAttribute("style"); h1[i].removeAttribute("class")
  };
  for (var i = 0; i < h2.length; i++){
      h2[i].removeAttribute("style"); h2[i].removeAttribute("class")
  };
  for (var i = 0; i < h3.length; i++){
      h3[i].removeAttribute("style"); h3[i].removeAttribute("class")
  };
  for (var i = 0; i < h4.length; i++){
      h4[i].removeAttribute("style"); h4[i].removeAttribute("class")
  };
  for (var i = 0; i < h5.length; i++){
      h5[i].removeAttribute("style"); h5[i].removeAttribute("class")
  };
  for (var i = 0; i < h6.length; i++){
      h6[i].removeAttribute("style"); h6[i].removeAttribute("class")
  };
  for (var i = 0; i < div.length; i++){
      div[i].removeAttribute("style"); div[i].removeAttribute("class")
  };

  return text
};

on('#ajax', 'input', '.smile_supported', function() {
    this.previousElementSibling.innerHTML = this.innerHTML.length
});

function send_category_data(form, url) {
  text_val1 = document.body.querySelector(".content_1");
  _val1 = format_text(text_val1);
  _text1 = _val1.innerHTML;

  $input = document.createElement("input");
  $input.setAttribute("name", "description");
  $input.setAttribute("type", "hidden");
  $input.classList.add("input_text");
  $input.value = _text1;
  form.append($input);
  form_data = new FormData(form);

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', url, true );
  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    ajax_get_reload(url)
  }};
  link.send(form_data);
};

function send_content_data(url) {
  text_val1 = document.body.querySelector(".content_1");
  _val1 = format_text(text_val1);
  _text1 = _val1.innerHTML;

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'GET', url + "?content=" + _text1, true );
  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    alert(_text1)
  } else {
    //console.log(link.responseText)
  }};
  link.send();
};

function send_post_data(form, url) {
  form_data = new FormData(form);
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'POST', url, true );
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    ajax_get_reload(url)
  } else { console.log(link.responseText) }};
  link.send(form_data);
};
function delete_item(url) {
  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
  link.open( 'GET', url, true );
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
  link.onreadystatechange = function () {
  if ( link.readyState == 4 && link.status == 200 ) {
    toast_success("??????????????!");
  }};
  link.send();
};
///////////SERVE //////////////////
on('body', 'click', '#create_serve_btn', function() {
  send_post_data(this.parentElement, "/create_serve/");
});
on('body', 'click', '#create_tech_category_btn', function() {
  send_category_data(this.parentElement, "/create_tech_categories/");
});
on('body', 'click', '#create_serve_category_btn', function() {
  send_category_data(this.parentElement, "/create_serve_categories/");
});
on('body', 'click', '#edit_serve_btn', function() {
  send_post_data(this.parentElement, "/edit_serve/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '#edit_text_work_btn', function() {
  send_content_data("/edit_content_serve/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '#edit_serve_category_btn', function() {
  send_category_data(this.parentElement, "/edit_serve_category/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '#edit_tech_category_btn', function() {
  send_category_data(this.parentElement, "/edit_tech_category/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '.remove_serve', function() {
  delete_item("/delete_serve/" + this.getAttribute("data-pk") + "/");
  this.parentElement.remove();
});
on('body', 'click', '.remove_serve_category', function() {
  delete_item("/delete_serve_category/" + this.getAttribute("data-pk") + "/");
  this.parentElement.remove();
});
on('body', 'click', '.remove_tech_category', function() {
  delete_item("/delete_tech_category/" + this.getAttribute("data-pk") + "/");
  this.parentElement.remove();
});

/////////////////////////////
on('body', 'click', '#create_work_btn', function() {
  send_post_data(this.parentElement, "/create_work/");
});
on('body', 'click', '#create_work_category_btn', function() {
  send_category_data(this.parentElement, "/create_work_categories/");
});
on('body', 'click', '#edit_work_btn', function() {
  send_post_data(this.parentElement, "/edit_work/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '#edit_text_work_btn', function() {
  send_content_data("/edit_content_work/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '#edit_work_category_btn', function() {
  send_category_data(this.parentElement, "/edit_work_category/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '.remove_work', function() {
  delete_item("/delete_work/" + this.getAttribute("data-pk") + "/");
  this.parentElement.remove();
});
on('body', 'click', '.remove_work_category', function() {
  delete_item("/delete_work_category/" + this.getAttribute("data-pk") + "/");
  this.parentElement.remove();
});

////////////////////////////////////

on('body', 'click', '#create_blog_btn', function() {
  send_post_data(this.parentElement, "/create_blog/");
});
on('body', 'click', '#edit_blog_btn', function() {
  send_post_data(this.parentElement, "/edit_blog/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '#edit_text_blog_btn', function() {
  send_content_data("/edit_content_blog/" + this.getAttribute("data-pk") + "/");
});

on('body', 'click', '#create_blog_category_btn', function() {
  send_category_data(this.parentElement, "/create_blog_categories/");
});
on('body', 'click', '#edit_blog_category_btn', function() {
  send_category_data(this.parentElement, "/edit_blog_category/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '.remove_blog', function() {
  delete_item("/delete_blog/" + this.getAttribute("data-pk") + "/");
  this.parentElement.remove();
});
on('body', 'click', '.remove_blog_category', function() {
  delete_item("/delete_blog_category/" + this.getAttribute("data-pk") + "/");
  this.parentElement.remove();
});

/////////////////////////////////////////

on('body', 'click', '#create_wiki_btn', function() {
  send_post_data(this.parentElement, "/create_wiki/");
});
on('body', 'click', '#create_wiki_category_btn', function() {
  send_category_data(this.parentElement, "/create_wiki_categories/");
});
on('body', 'click', '#edit_wiki_btn', function() {
  send_post_data(this.parentElement, "/edit_wiki/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '#edit_text_wiki_btn', function() {
  send_content_data("/edit_content_wiki/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '#edit_wiki_category_btn', function() {
  send_category_data(this.parentElement, "/edit_wiki_category/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '.remove_wiki', function() {
  delete_item("/delete_wiki/" + this.getAttribute("data-pk") + "/");
  this.parentElement.remove();
});
on('body', 'click', '.remove_wiki_category', function() {
  delete_item("/delete_wiki_category/" + this.getAttribute("data-pk") + "/");
  this.parentElement.remove();
});

 //////////////////////////////////

on('body', 'click', '#create_store_btn', function() {
  send_post_data(this.parentElement, "/create_store/");
});
on('body', 'click', '#create_store_category_btn', function() {
  send_category_data(this.parentElement, "/create_store_categories/");
});
on('body', 'click', '#edit_store_btn', function() {
  send_post_data(this.parentElement, "/edit_store/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '#edit_text_store_btn', function() {
  send_content_data("/edit_content_store/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '#edit_store_category_btn', function() {
  send_category_data(this.parentElement, "/edit_store_category/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '.remove_store', function() {
  delete_item("/delete_store/" + this.getAttribute("data-pk") + "/");
  this.parentElement.remove();
});
on('body', 'click', '.remove_store_category', function() {
  delete_item("/delete_store_category/" + this.getAttribute("data-pk") + "/");
  this.parentElement.remove();
});

////////////////////////////////////

on('body', 'click', '#create_service_btn', function() {
  send_post_data(this.parentElement, "/create_service/");
});
on('body', 'click', '#create_service_category_btn', function() {
  send_category_data(this.parentElement, "/create_service_categories/");
});
on('body', 'click', '#edit_service_btn', function() {
  send_post_data(this.parentElement, "/edit_service/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '#edit_text_service_btn', function() {
  send_content_data("/edit_content_service/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '#edit_service_category_btn', function() {
  send_category_data(this.parentElement, "/edit_service_category/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '.remove_service', function() {
  delete_item("/delete_service/" + this.getAttribute("data-pk") + "/");
  this.parentElement.remove();
});
on('body', 'click', '.remove_service_category', function() {
  delete_item("/delete_service_category/" + this.getAttribute("data-pk") + "/");
  this.parentElement.remove();
});

///////////////////////////

on('body', 'click', '#create_tag_btn', function() {
  send_post_data(this.parentElement, "/create_tag/");
});
on('body', 'click', '#edit_tag_btn', function() {
  send_post_data(this.parentElement, "/edit_tag/" + this.getAttribute("data-pk") + "/");
});
on('body', 'click', '.remove_tag', function() {
  delete_item("/delete_tag/" + this.getAttribute("data-pk") + "/");
  this.parentElement.remove();
});
