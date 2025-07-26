use clap::{Arg, ArgAction};

pub struct ArgManager;
impl ArgManager {
    pub fn base(&self, name: &'static str, req: bool, about: &'static str) -> Arg {
        let arg = Arg::new(name).help(about);
        if req {
            arg.required(true)
        } else {
            arg
        }
    }

    pub fn flag_long(&self, name: &'static str, req: bool, about: &'static str) -> Arg {
        self.base(name, req, about).long(name)
    }

    pub fn flag_short(
        &self,
        name: &'static str,
        req: bool,
        about: &'static str,
        short: char,
    ) -> Arg {
        self.flag_long(name, req, about).short(short)
    }

    pub fn base_val(
        &self,
        name: &'static str,
        req: bool,
        about: &'static str,
        default_value: &'static str,
    ) -> Arg {
        self.base(name, req, about).default_value(default_value)
    }

    pub fn flag_long_val(
        &self,
        name: &'static str,
        req: bool,
        about: &'static str,
        default_value: &'static str,
    ) -> Arg {
        self.flag_long(name, req, about)
            .default_value(default_value)
    }

    pub fn flag_short_val(
        &self,
        name: &'static str,
        req: bool,
        about: &'static str,
        short: char,
        default_value: &'static str,
    ) -> Arg {
        self.flag_short(name, req, about, short)
            .default_value(default_value)
    }

    fn get_action(&self, value: bool) -> ArgAction {
        if value {
            ArgAction::SetFalse
        } else {
            ArgAction::SetTrue
        }
    }

    pub fn flag_long_bool(
        &self,
        name: &'static str,
        req: bool,
        about: &'static str,
        default_value: bool,
    ) -> Arg {
        self.flag_long(name, req, about)
            .action(self.get_action(default_value))
    }

    pub fn flag_short_bool(
        &self,
        name: &'static str,
        req: bool,
        about: &'static str,
        short: char,
        default_value: bool,
    ) -> Arg {
        self.flag_short(name, req, about, short)
            .action(self.get_action(default_value))
    }
}
