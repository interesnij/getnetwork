
function get_document_opacity_0() {
  document.body.style.overflowY = "hidden";
  document.body.style.marginRight = "20px";
  overlay = document.body.querySelector(".body_overlay");
  overlay.style.visibility = "unset";
  overlay.style.opacity = "1";
};
function get_document_opacity_1() {
  document.body.style.overflowY = "scroll";
  document.body.style.marginRight = "0";
  overlay = document.body.querySelector(".body_overlay");
  overlay.style.visibility = "hidden";
  overlay.style.opacity = "0";
};

function close_fullscreen() {
  container = document.body.querySelector("#fullscreens_container");
  if (!container.innerHTML) {
    get_document_opacity_1();
    return
  };
  container = document.body.querySelector("#fullscreens_container");
  _window = container.querySelector(".card_fullscreen");
  _window.remove();

  if (!container.innerHTML) {
    get_document_opacity_1();
  } else {
    prev_window = container.querySelector(".card_fullscreen");
    prev_window.querySelector(".this_fullscreen_hide").style.display = "unset";
  };
};

function create_fullscreen(url, type_class) {
  container = document.body.querySelector("#fullscreens_container");

  if (container.innerHTML) {
    prev_window = container.querySelector(".card_fullscreen");
    prev_window.querySelector(".this_fullscreen_hide").style.display = "none";
  };

  try {
    count_items = container.querySelectorAll(".card_fullscreen").length + 1
  } catch {count_items = 0};

  $parent_div = document.createElement("div");
  $parent_div.classList.add("card_fullscreen", "mb-30", "border", type_class);
  $parent_div.style.zIndex = 100 + count_items;
  $parent_div.style.opacity = "0";

  if (document.body.querySelector(".desctop_nav")) {
    hide_svg = '<svg class="svg_default" style="position:fixed;" width="30" height="30" fill="currentColor" viewBox="0 0 24 24"><path d="M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z"/><path d="M0 0h24v24H0z" fill="none"/></svg>'
  } else { hide_svg = "" };
  $hide_span = document.createElement("span");
  $hide_span.classList.add("this_fullscreen_hide");
  $loader = document.createElement("div");

  $loader.setAttribute("id", "fullscreen_loader");
  $hide_span.innerHTML = hide_svg;
  $parent_div.append($hide_span);
  $parent_div.append($loader);
  container.prepend($parent_div);

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
  link.open('GET', url, true);
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link.onreadystatechange = function() {
      if (this.readyState == 4 && this.status == 200) {
          elem = link.responseText;

          $loader.innerHTML = elem;
          height = $loader.scrollHeight*1 + 30;
          if (height < 500 && !$loader.querySelector(".data_display")) {
            $parent_div.style.height = height + "px";
            $loader.style.overflowY = "unset";

            _height = (window.innerHeight - height - 50) / 2;
            $parent_div.style.top = _height + "px";
            prev_next_height = _height*1 + 50 + "px";
          } else {
            $parent_div.style.height = "100%";
            $parent_div.style.top = "15px";
            $loader.style.overflowY = "auto";
          };
          $parent_div.style.opacity = "1";
          if ($loader.querySelector(".data_display")) {
            $loader.style.overflowY = "unset";
          }

          get_document_opacity_0();

          $loader.onscroll = function() {
            if ($loader.querySelector(".next_page_list")) {
              box = $loader.querySelector('.next_page_list');
              if (box && box.classList.contains("next_page_list")) {
                  inViewport = elementInViewport(box);
                  if (inViewport) {
                      box.remove();
                      var link_3 = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
                      link_3.open('GET', location.protocol + "//" + location.host + box.getAttribute("data-link"), true);
                      link_3.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

                      link_3.onreadystatechange = function() {
                          if (this.readyState == 4 && this.status == 200) {
                              var elem = document.createElement('span');
                              elem.innerHTML = link_3.responseText;
                              $loader.querySelector(".is_block_paginate").insertAdjacentHTML('beforeend', elem.querySelector(".is_block_paginate").innerHTML);
                            }
                      }
                      link_3.send();
                  }
              };
            };
          };
      }
  };
  link.send();
};

function change_this_fullscreen(_this, type_class) {
  _this.parentElement.classList.contains("col") ? $loader = _this.parentElement.parentElement.parentElement.parentElement : $loader = _this.parentElement.parentElement;
  $loader.innerHTML = "";
  $parent_div.style.opacity = "0";
  $parent_div.style.height = "35px";
  url = _this.getAttribute("href");

  link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
  link.open('GET', url, true);
  link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

  link.onreadystatechange = function() {
      if (this.readyState == 4 && this.status == 200) {
          elem = link.responseText;
          $loader.innerHTML = elem;
          height = $loader.scrollHeight*1 + 30;
          $parent_div = $loader.parentElement
          if (height < 500 && !$loader.querySelector(".data_display")){
            $parent_div.style.height = height + "px";
            _height = (window.innerHeight - height - 50) / 2;
            $parent_div.style.top = _height + "px";
            prev_next_height = _height*1 + 50 + "px";
            $loader.style.overflowY = "unset";
          } else {
            $parent_div.style.height = "100%";
            $parent_div.style.top = "15px";
            $loader.style.overflowY = "auto";
          };
          $parent_div.style.opacity = "1";
          $parent_div.style.opacity = "1";
          if ($loader.querySelector(".data_display")) {
            $loader.style.overflowY = "unset";
          };

          get_document_opacity_0();

          $loader.onscroll = function() {
            if ($loader.querySelector(".next_page_list")) {
              box = $loader.querySelector('.next_page_list');
              if (box && box.classList.contains("next_page_list")) {
                  inViewport = elementInViewport(box);
                  if (inViewport) {
                      box.remove();
                      var link_3 = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject('Microsoft.XMLHTTP');
                      link_3.open('GET', location.protocol + "//" + location.host + box.getAttribute("data-link"), true);
                      link_3.setRequestHeader('X-Requested-With', 'XMLHttpRequest');

                      link_3.onreadystatechange = function() {
                          if (this.readyState == 4 && this.status == 200) {
                              var elem = document.createElement('span');
                              elem.innerHTML = link_3.responseText;
                              $loader.querySelector(".is_block_paginate").insertAdjacentHTML('beforeend', elem.querySelector(".is_block_paginate").innerHTML);
                            }
                      }
                      link_3.send();
                  }
              };
            };
          }
      }
  };
  link.send();
};

function addStyleSheets(href) {
    $head = document.head, $link = document.createElement('link');
    $link.rel = 'stylesheet';
    $link.classList.add("color");
    $link.href = href;
    $head.appendChild($link)
};


class ToastManager {
    constructor() {
        this.id = 0;
        this.toasts = [];
        this.icons = {
            'SUCCESS': "",
            'ERROR': '',
            'INFO': '',
            'WARNING': '',
        };
        var body = document.querySelector('#ajax');
        this.toastsContainer = document.createElement('div');
        this.toastsContainer.classList.add('toasts', 'border-0');
        body.appendChild(this.toastsContainer)
    }
    showSuccess(message) {
        return this._showToast(message, 'SUCCESS')
    }
    showError(message) {
        return this._showToast(message, 'ERROR')
    }
    showInfo(message) {
        return this._showToast(message, 'INFO')
    }
    showWarning(message) {
        return this._showToast(message, 'WARNING')
    }
    _showToast(message, toastType) {
        var newId = this.id + 1;
        var newToast = document.createElement('div');
        newToast.style.display = 'inline-block';
        newToast.classList.add(toastType.toLowerCase());
        newToast.classList.add('toast');
        newToast.innerHTML = `<progress max="100"value="0"></progress><h3>${message}</h3>`;
        var newToastObject = {
            id: newId,
            message,
            type: toastType,
            timeout: 4000,
            progressElement: newToast.querySelector('progress'),
            counter: 0,
            timer: setInterval(() => {
                newToastObject.counter += 1000 / newToastObject.timeout;
                newToastObject.progressElement.value = newToastObject.counter.toString();
                if (newToastObject.counter >= 100) {
                    newToast.style.display = 'none';
                    clearInterval(newToastObject.timer);
                    this.toasts = this.toasts.filter((toast) => {
                        return toast.id === newToastObject.id
                    })
                }
            }, 10)
        }
        newToast.addEventListener('click', () => {
            newToast.style.display = 'none';
            clearInterval(newToastObject.timer);
            this.toasts = this.toasts.filter((toast) => {
                return toast.id === newToastObject.id
            })
        });
        this.toasts.push(newToastObject);
        this.toastsContainer.appendChild(newToast);
        return this.id++
    }
}

function toast_success(text) {
    var toasts = new ToastManager();
    toasts.showSuccess(text)
}

function toast_error(text) {
    var toasts = new ToastManager();
    toasts.showError(text)
}

function toast_info(text) {
    var toasts = new ToastManager();
    toasts.showInfo(text)
}

function toast_warning(text) {
    var toasts = new ToastManager();
    toasts.showWarning(text)
}

on('#ajax', 'click', '.select_serve', function(event) {
  _this = this;
  if (event.target.classList.contains("get_object_info")) {
    return
  };
  counter = document.body.querySelector(".total_price_counter");
  counter_serve_price = counter.getAttribute("data-serve")*1;
  counter_serve_list = counter.parentElement
    .getAttribute("data-servelist")
    .replace('[', "")
    .replace(']', "")
    .split(',');
  serve_pk = _this.querySelector(".get_object_info").getAttribute("data-pk");

  // ?????? ???????????? ???? ???????????? ?????????????????? ?????????? ???? ????????????????
  // ?????????????????? ?????????????????? (???????? ?????????????????? "??????. ????????????????????")
  // ?? ????, ?????????????? ?????? ???? ?????????????????? ??????????????, ??????????????.
  tab_panes = _this.parentElement.parentElement.parentElement.parentElement.querySelectorAll(".tab-pane");
  for (var i = 0; i < tab_panes.length; i++){
    // ???????????????? ???????????? ?? ???????? ?????? ????????????????, ?????????????? ???? ??????????.
    if (!tab_panes[i].classList.contains("active")){
      serve_list = tab_panes[i].querySelectorAll(".select_serve");
      for (var i2 = 0; i2 < serve_list.length; i2++){
        // ?????????? ?????????? ?????????????????? ?????????????? ???????? ???? ?????????? ???????? ?????????????????? ?????????? ?? ????????????
        // ????????????????. ?? ?????????? ?????????????????? "data-serve" ????????????????
          if (!serve_list[i2].classList.contains("is_default") && serve_list[i2].classList.contains("hover")){
            serve_list[i2].classList.remove("hover");
            _serve_price = serve_list[i2].querySelector(".price").innerHTML*1
            counter.innerHTML = counter.innerHTML*1 - _serve_price;
            counter.setAttribute("data-serve", counter_serve_price - _serve_price);
            index = counter_serve_list.indexOf(serve_pk);
            if (index > -1) {
              counter_serve_list.splice(index, 1);
            }
          }
      };
    };
  };

  counter.parentElement.setAttribute("data-servelist", counter_serve_list)
  // ???????????? ???????? ?????????? ?? ?????????????? ???????? ????????????
  serve_price = _this.querySelector(".price").innerHTML*1

  if (!_this.classList.contains("hover")){
    // ???????? ???? ?????????????? ?????????? ???? ??????????????...
    counter.innerHTML = counter.innerHTML*1 + serve_price;
    counter.setAttribute("data-serve", counter_serve_price + serve_price);
    _this.classList.add("hover");
    _this.querySelector(".action_text").innerHTML = '??????????????';
    counter_serve_list.push(serve_pk);
    counter.parentElement.setAttribute("data-servelist", counter_serve_list)
  }
  else {
    // ???????? ?????????? ??????????????, ???????? ?????????? ?????????????????? ?? ?????????????? ?????????????????? ???? ?????????? ??????????.
    // ?? ?????????? ?????????????????? "data-serve" ????????????????
    counter.innerHTML = counter.innerHTML*1 - serve_price;
    counter.setAttribute("data-serve", counter_serve_price - serve_price);
    _this.classList.remove("hover");
    _this.querySelector(".action_text").innerHTML = '??????????????';
    var index = counter_serve_list.indexOf(serve_pk);
    if (index > -1) {
      counter_serve_list.splice(index, 1);
      counter.parentElement.setAttribute("data-servelist", counter_serve_list)
    }
  }
});

function service_tab_action(_this, tab_class){
  counter = document.body.querySelector(".total_price_counter");
  if (!_this.classList.contains("active")){
    // ?????????????????????? ?????????????? ????????
    if (_this.getAttribute("data-sum")){
      counter.innerHTML = counter.innerHTML*1 - counter.getAttribute("data-oldsum")*1 + _this.getAttribute("data-sum")*1;
      counter.setAttribute("data-oldsum", _this.getAttribute("data-sum"));
    };
    nav = _this.parentElement.parentElement.parentElement;
    nav_items = nav.querySelectorAll(".yy");
    for (var i = 0; i < nav_items.length; i++){
      nav_items[i].classList.remove("active", "in");
      nav_items[i].classList.add("pointer")
    };
    _this.classList.add("active", "in");
    _this.classList.remove("pointer");
    tabs = nav.querySelector(".tab-content");

    tabs_panes = tabs.querySelectorAll(".tab-pane");
    for (var i = 0; i < tabs_panes.length; i++){
        serve_list = tabs_panes[i].querySelectorAll(".select_serve");
        for (var i2 = 0; i2 < serve_list.length; i2++){
          // ?????????? ?????????? ?????????????????? ?????????????? ???????? ???? ?????????? ???????? ?????????????????? ?????????? ?? ????????????
          // ????????????????. ?? ?????????? ?????????????????? "data-serve" ????????????????
            if (!serve_list[i2].classList.contains("is_default") && serve_list[i2].classList.contains("hover")){
              serve_list[i2].classList.remove("hover");
            }
        };
      tabs_panes[i].classList.remove("active", "in")
    };

    cur = tabs.querySelector(tab_class);
    cur.classList.add("active", "in");
    // ???? ?????????????????????? ??????, ?? ???????????? ?????? ???????????? ?? ?????????????????? ????????????,
    // ???? ?????????? ?? ids ???? ???????????? ?? ?????????????? ?????????? - "data-serve" 0, ?? ids ?? ???????????? ?????????????????? ????????.

    counter.setAttribute("data-serve", 0);
    new_serve_list = cur.querySelectorAll(".is_default");
    new_arr = [];
    for (var i3 = 0; i3 < new_serve_list.length; i3++){
      new_arr.push(new_serve_list[i3].querySelector(".get_object_info").getAttribute("data-pk"))
    };
    counter.parentElement.setAttribute("data-servelist", new_arr);
    counter.innerHTML = cur.getAttribute("data-sum");
  }
};

function on(elSelector, eventName, selector, fn) {var element = document.querySelector(elSelector);element.addEventListener(eventName, function(event) {var possibleTargets = element.querySelectorAll(selector);var target = event.target;for (var i = 0, l = possibleTargets.length; i < l; i++) {var el = target;var p = possibleTargets[i];while (el && el !== element) {if (el === p) {return fn.call(p, event);}el = el.parentNode;}}});};

function get_custom_design() {
  color = "white";
  params = window.location.search.replace( '?', '').split('&');

    if (params[0] && params[0].split("=")[0] == "f") {
      color = params[0].split("=")[1]}
    else if (params[1] && params[1].split("=")[0] == "f") {
      color = params[1].split("=")[1]
    } else if (params[2] && params[2].split("=")[0] == "f") {
      color = params[2].split("=")[1]
    };

    addStyleSheets("/static/styles/color/" + color + ".css")
    btn = document.body.querySelector(".anon_color_change");
    btn.setAttribute("data-color", color)
};
get_custom_design();

on('body', 'click', '.ajax', function(event) {
  event.preventDefault();
  if (this.getAttribute("href") == window.location.pathname){
    toast_info("???? ?????? ???? ???????? ????????????????");
    return
  };
  ajax_get_reload(this.getAttribute("href"))
});


on('#ajax', 'click', '.s_1', function() {
  service_tab_action(this, ".tab_1")
});
on('#ajax', 'click', '.s_2', function() {
  service_tab_action(this, ".tab_2")
});
on('#ajax', 'click', '.s_3', function() {
  service_tab_action(this, ".tab_3")
});
on('#ajax', 'click', '.s_4', function() {
  service_tab_action(this, ".tab_4")
});
on('#ajax', 'click', '.s_5', function() {
  service_tab_action(this, ".tab_5")
});
on('#ajax', 'click', '.s_6', function() {
  service_tab_action(this, ".tab_6")
});
on('#ajax', 'click', '.s_7', function() {
  service_tab_action(this, ".tab_7")
});

on('body', 'click', '.anon_color_change', function() {
  color = this.getAttribute("data-color");
  if (color == "white") {
    addStyleSheets("/static/styles/color/dark.css");
    this.setAttribute("data-color", "dark");
    new_color = "dark"
  } else if (color == "dark") {
    addStyleSheets("/static/styles/color/yellow.css");
    this.setAttribute("data-color", "yellow");
    new_color = "yellow"
  } else if (color == "yellow") {
    addStyleSheets("/static/styles/color/white_kletka.css");
    this.setAttribute("data-color", "white_kletka");
    new_color = "white_kletka"
  } else if (color == "white_kletka") {
    addStyleSheets("/static/styles/color/dark_wood.css");
    this.setAttribute("data-color", "dark_wood");
    new_color = "dark_wood"
  } else if (color == "dark_wood") {
    addStyleSheets("/static/styles/color/white.css");
    this.setAttribute("data-color", "white");
    new_color = "white"
  };
  _href = window.location.href;
  _search = window.location.search;
  _params = _search.replace( '?', '').split('&');
  if (_search.indexOf('f=') !== -1){
    r = new URL(_href);
    r.searchParams.delete('f');
    __url = r;
  } else { __url = _href };
  if (_params[1]) {
    _url = __url + "&f=" + new_color;
  } else {
    _url = __url + "?f=" + new_color;
  };
  window.history.replaceState(null, null, _url);
});
on('body', 'click', '.this_fullscreen_hide', function() {
  close_fullscreen()
});
on('body', 'click', '.body_overlay', function() {
  close_fullscreen()
});

on('#ajax', 'click', '.get_object_info', function() {
  create_fullscreen("/load_item/?_object_type=" + this.getAttribute("data-type") + "&_owner_type=" + this.getAttribute("owner-type") + "&_object_pk=" + this.getAttribute("data-pk") + "&_owner_pk=" + this.getAttribute("owner-pk"), "worker_fullscreen");
});

on('body', 'click', '.next_item', function(event) {
  event.preventDefault();
  this.style.display = "none";
  change_this_fullscreen(this, document.getElementById('item_loader'));
});
on('body', 'click', '.prev_item', function(event) {
  event.preventDefault();
  this.style.display = "none";
  change_this_fullscreen(this, document.getElementById('item_loader'));
});

on('body', 'input', '.general_search', function() {
    _this = this;
    _href = window.location.href;
    _search = window.location.search;
    _params = _search.replace( '?', '').split('&');

    if (_this.classList.contains("search-field") && !document.body.querySelector(".search_section")) {
        r = new URL("http://" + location.host + "/search/" + _search);
        if (_search.indexOf('q=') !== -1){
          r.searchParams.delete('q')
        };
        r.searchParams.append('q', _this.value);
      ajax_get_reload(r)
    }
    else if (document.body.querySelector(".search_section")) {
      if (_this.getAttribute("data-folder")) {
        folder = _this.getAttribute("data-folder")
      } else {
        folder = ""
      };
      r = new URL("http://" + location.host + "/search" + folder + "/" + _search);
      if (_search.indexOf('q=') !== -1){
        r.searchParams.delete('q')
      };
      r.searchParams.append('q', _this.value);
    var ajax_link = window.XMLHttpRequest ? new XMLHttpRequest() : new ActiveXObject( 'Microsoft.XMLHTTP' );
      ajax_link.open( 'GET', r, true );
      ajax_link.setRequestHeader('X-Requested-With', 'XMLHttpRequest');
      ajax_link.onreadystatechange = function () {
        if ( this.readyState == 4 && this.status == 200 ) {
          document.body.querySelector(".search_page").value = _this.value;
          elem_ = document.createElement('span');
          elem_.innerHTML = ajax_link.responseText;
          search = elem_.querySelector(".search_section");
          div = document.body.querySelector(".search_section");
          div.innerHTML = '';
          div.innerHTML = search.innerHTML;
          document.title = elem_.querySelector("title").innerHTML;
          window.history.replaceState(null, null, r);
        }
      }
      ajax_link.send();
  }
});

on('#ajax', 'click', '.show_tech_category', function() {
  next_div = this.nextElementSibling;
  counter = document.body.querySelector(".total_price_counter")
  if (next_div.classList.contains("hidden")) {
    counter.innerHTML = counter.innerHTML*1 + next_div.querySelector(".tab_1").getAttribute("data-sum")*1;
  } else {
    counter.innerHTML = counter.innerHTML*1 - next_div.querySelector(".tab_1").getAttribute("data-sum")*1;
  }
  this.querySelector(".cat_description").classList.toggle("hidden");
  this.querySelector(".cat_name").classList.toggle("hidden")
  next_div.classList.toggle("hidden")
});
