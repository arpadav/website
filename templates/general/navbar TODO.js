import {readJSON} from '../../../public/js/technicals.js';
readJSON('../../../views/partials/header.json', null, create_header_html);

// read from header.json
function create_header_html(params, json_arr) {
  let header_html = '<ul>'

  let pages = json_arr['pages'];
  for (let i = 0; i < pages.length; i++) {
    header_html = header_html + '<li><a ';
    header_html = header_html + 'href=\"../../../' + pages[i]['dir'] + '\" target=\"_parent\">' + pages[i]['display'] + '</a></li>';
  }
  document.getElementById('header').innerHTML = header_html + '</ul>';
}
