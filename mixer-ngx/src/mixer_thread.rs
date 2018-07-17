

use std::thread ;

use ngx_rust::bindings::ngx_int_t;
use ngx_rust::bindings::NGX_OK;

use mixer_report::mixer_report_background;



// start background activities
#[no_mangle]
pub extern fn nginmesh_mixer_init() -> ngx_int_t {

    ngx_event_debug!("init mixer start ");
    // Spawn new thread to listen for mixer attributes from NGINX and send to mixer
    thread::spawn(|| {
        ngx_event_debug!("starting mixer report background task");
        mixer_report_background();
    });


    ngx_event_debug!("init mixer end ");
    return NGX_OK as ngx_int_t;
}

#[no_mangle]
pub extern fn nginmesh_mixer_exit() {

    ngx_event_debug!("mixer exit ");
}

