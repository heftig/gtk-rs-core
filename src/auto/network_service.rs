// This file was generated by gir (https://github.com/gtk-rs/gir @ fbb95f4)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use SocketConnectable;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct NetworkService(Object<ffi::GNetworkService, ffi::GNetworkServiceClass>): SocketConnectable;

    match fn {
        get_type => || ffi::g_network_service_get_type(),
    }
}

impl NetworkService {
    pub fn new(service: &str, protocol: &str, domain: &str) -> NetworkService {
        unsafe {
            from_glib_full(ffi::g_network_service_new(service.to_glib_none().0, protocol.to_glib_none().0, domain.to_glib_none().0))
        }
    }
}

pub trait NetworkServiceExt {
    fn get_domain(&self) -> Option<String>;

    fn get_protocol(&self) -> Option<String>;

    fn get_scheme(&self) -> Option<String>;

    fn get_service(&self) -> Option<String>;

    fn set_scheme(&self, scheme: &str);

    fn connect_property_domain_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_protocol_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_scheme_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_service_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<NetworkService> + IsA<glib::object::Object>> NetworkServiceExt for O {
    fn get_domain(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_network_service_get_domain(self.to_glib_none().0))
        }
    }

    fn get_protocol(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_network_service_get_protocol(self.to_glib_none().0))
        }
    }

    fn get_scheme(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_network_service_get_scheme(self.to_glib_none().0))
        }
    }

    fn get_service(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_network_service_get_service(self.to_glib_none().0))
        }
    }

    fn set_scheme(&self, scheme: &str) {
        unsafe {
            ffi::g_network_service_set_scheme(self.to_glib_none().0, scheme.to_glib_none().0);
        }
    }

    fn connect_property_domain_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::domain",
                transmute(notify_domain_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_protocol_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::protocol",
                transmute(notify_protocol_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_scheme_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::scheme",
                transmute(notify_scheme_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_service_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::service",
                transmute(notify_service_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_domain_trampoline<P>(this: *mut ffi::GNetworkService, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<NetworkService> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&NetworkService::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_protocol_trampoline<P>(this: *mut ffi::GNetworkService, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<NetworkService> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&NetworkService::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_scheme_trampoline<P>(this: *mut ffi::GNetworkService, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<NetworkService> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&NetworkService::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_service_trampoline<P>(this: *mut ffi::GNetworkService, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<NetworkService> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&NetworkService::from_glib_borrow(this).downcast_unchecked())
}
