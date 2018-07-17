
use std::collections::HashMap;
use protobuf::well_known_types::Timestamp;
use ngx_rust::bindings:: { ngx_http_request_s, ngx_http_headers_out_t} ;
use ngx_mixer_transport::attribute::attr_wrapper::AttributeWrapper;
use ngx_mixer_transport::attribute::global_dict::{ REQUEST_HEADER, REQUEST_HOST, REQUEST_METHOD, REQUEST_PATH,
                                                   REQUEST_REFER, REQUEST_SCHEME, REQUEST_SIZE, REQUEST_TIME, REQUEST_USERAGENT,
                                                   SOURCE_UID, SRC_UID_HEADER, RESPONSE_CODE, RESPONSE_SIZE, RESPONSE_HEADERS
};

use super::config::MixerConfig;


impl MixerConfig for ngx_http_request_s  {



    fn process_istio_attr(&self, attr: &mut AttributeWrapper )  {

        ngx_http_debug!(self,"send request attribute to mixer");

        let headers_in = self.headers_in;


        attr.insert_string_attribute(REQUEST_HOST,  headers_in.host_str());
        attr.insert_string_attribute(REQUEST_METHOD, self.method_name.to_str());
        attr.insert_string_attribute(REQUEST_PATH, self.uri.to_str());

        let referer = headers_in.referer_str();
        if let Some(ref_str) = referer {
            attr.insert_string_attribute(REQUEST_REFER, ref_str);
        }

        //let scheme = request.http_protocol.to_str();
        attr.insert_string_attribute(REQUEST_SCHEME, "http"); // hard code now


        attr.insert_int64_attribute(REQUEST_SIZE, self.request_length);

        let mut request_time = Timestamp::new();
        request_time.set_seconds(self.start_sec);
        request_time.set_nanos(self.start_msec as i32);
        attr.insert_time_stamp_attribute(REQUEST_TIME, request_time);

        attr.insert_string_attribute(REQUEST_USERAGENT, headers_in.user_agent_str());


        // fill in the string value
        let mut map: HashMap<String,String> = HashMap::new();
        {
            for (name,value) in headers_in.headers_iterator()   {
                ngx_http_debug!(self,"in header name: {}, value: {}",&name,&value);

                match name.as_ref()  {

                    SRC_UID_HEADER => {
                        ngx_http_debug!(self,"source UID received {}",&value);
                        attr.insert_string_attribute( SOURCE_UID,&value);
                    },
                    _ => {
                        ngx_http_debug!(self,"other source header {}",&name);
                        map.insert(name,value);
                    }
                }


            }
        }

        attr.insert_string_map(REQUEST_HEADER, map);

    }
}



impl MixerConfig for ngx_http_headers_out_t {

    fn process_istio_attr(&self, attr: &mut AttributeWrapper, )  {


        // ngx_http_debug!("send request header attribute to mixer");
        // Add response code and response size from request
        attr.insert_int64_attribute(RESPONSE_CODE, self.status as i64);
        attr.insert_int64_attribute(RESPONSE_SIZE, self.content_length_n);

        // fill in the string value from all values of request struct
        let mut map: HashMap<String,String> = HashMap::new();
        {
            for (name,value) in self.headers_iterator()   {

                map.insert(name,value);

            }
        }
        // Add response headers
        attr.insert_string_map(RESPONSE_HEADERS, map);
    }
}


