#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
/*!
Low-level bindings to the Gmsh SDK.

Gmsh C API bindings (v4.13.1)

Gmsh is copyright (C) 1997-2019 by C. Geuzaine and J.-F. Remacle

By design, the Gmsh C API is purely functional, and only uses elementary types.
!*/

#![doc(html_logo_url = "https://gitlab.onelab.info/gmsh/gmsh/raw/master/utils/icons/gmsh.svg")]

// include rust interface generated from gmshc.h
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use std::ffi::CString;
    use std::os::raw::c_int;

    #[test]
    pub fn sanity() {
        // open gmsh
        let exec_name = CString::new("gmsh").unwrap();
        let mut ierr: c_int = 0;
        unsafe {
            crate::gmshInitialize(
                1 as c_int, // argc
                [exec_name.into_raw()].as_mut_ptr(), // argv
                0 as c_int, // readFromConfig
                0 as c_int, // run (added after mgsh 4.4.x)
                &mut ierr,
            );
        }

        // close gmsh
        let mut ierr: c_int = 0;
        unsafe {
            crate::gmshFinalize(&mut ierr);
        }
    }
}
