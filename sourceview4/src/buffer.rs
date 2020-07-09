// buffer.rs
//
// Copyright 2020 Bilal Elmoussaoui <bil.elmoussaoui@gmail.com>
//
// This file is free software; you can redistribute it and/or modify it
// under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation; either version 2.1 of the
// License, or (at your option) any later version.
//
// This file is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this program.  If not, see <http://www.gnu.org/licenses/>.
//
// SPDX-License-Identifier: LGPL-2.1-or-later

use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use std::mem::transmute;

use glib::translate::*;
use gtk;
use gtk_source_sys;
use std::boxed::Box as Box_;

use BracketMatchType;
use Buffer;

pub trait BufferExtManual: 'static {
    fn connect_bracket_matched<F: Fn(&Self, Option<&gtk::TextIter>, BracketMatchType) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<Buffer>> BufferExtManual for O {
    fn connect_bracket_matched<F: Fn(&Self, Option<&gtk::TextIter>, BracketMatchType) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn bracket_matched_trampoline<
            P,
            F: Fn(&P, Option<&gtk::TextIter>, BracketMatchType) + 'static,
        >(
            this: *mut gtk_source_sys::GtkSourceBuffer,
            iter: *mut gtk_sys::GtkTextIter,
            state: gtk_source_sys::GtkSourceBracketMatchType,
            f: glib_sys::gpointer,
        ) where
            P: IsA<Buffer>,
        {
            let f: &F = &*(f as *const F);
            f(
                &Buffer::from_glib_borrow(this).unsafe_cast_ref(),
                Option::<gtk::TextIter>::from_glib_borrow(iter).as_ref(),
                from_glib(state),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"bracket-matched\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    bracket_matched_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}
