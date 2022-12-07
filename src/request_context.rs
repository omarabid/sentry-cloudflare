use crate::consts;
use crate::error::Result;
use std::ops::Deref;
use worker::kv;

//use crate::{consts, route, sentry, utils::RequestExt};

pub struct RequestContext {
    req: worker::Request,
    //env: worker::Env,
    ctx: worker::Context,
    //route: route::Route,
}

impl RequestContext {
    pub fn new(req: worker::Request, ctx: worker::Context) -> Self {
        //let route = route::Route::new(&req);
        Self {
            req,
            //env,
            ctx,
            //route,
        }
    }

    pub fn req(&self) -> &worker::Request {
        &self.req
    }

    pub fn req_mut(&mut self) -> &mut worker::Request {
        &mut self.req
    }

    //#[allow(dead_code)]
    //pub fn env(&self) -> &worker::Env {
    //&self.env
    //}

    pub fn ctx(&self) -> &worker::Context {
        &self.ctx
    }

    //pub fn route(&self) -> &route::Route {
    //&self.route
    //}

    //pub fn transaction(&self) -> String {
    //use route::{Api::*, Route::*};
    //match self.route {
    //Asset => "asset".to_owned(),
    //App(ref app) => format!("app::{}", <&str>::from(app)),
    //Api(ref api) => match api {
    //Get(ref get) => format!("api::get::{}", <&str>::from(get)),
    //Post(ref post) => format!("api::post::{}", <&str>::from(post)),
    //Delete(ref delete) => format!("api::delete::{}", <&str>::from(delete)),
    //},
    //NotFound => "not_found".to_owned(),
    //}
    //}
    pub fn transaction(&self) -> String {
        String::from("transaction id")
    }

    pub fn get_sentry_options(&self) -> Option<crate::Options> {
        //let project = self.env.var(consts::ENV_SENTRY_PROJECT).ok()?.to_string();
        //let token = self.env.var(consts::ENV_SENTRY_TOKEN).ok()?.to_string();
        let project = "4504099338518528".to_owned();
        let token = "d7604c9922ef4faf94c163a13d3acbe4".to_owned();

        Some(crate::Options { project, token })
    }

    pub async fn get_sentry_user(&self) -> crate::User {
        crate::User {
            //username: self
            //   .session()
            //   .await
            //  .ok()
            //  .flatten()
            //  .map(|user| user.name.into()),
            username: None,
            ip_address: self.req.headers().get("cf-connecting-ip").ok().flatten(),
            country: self.req.headers().get("cf-ipcountry").ok().flatten(),
        }
    }

    pub async fn get_sentry_request(&self) -> crate::Request {
        crate::Request {
            url: self.req.url().ok(),
            method: Some(self.req.inner().method()),
            headers: self.req.headers().into_iter().collect(),
            ..Default::default()
        }
    }

    //pub fn get_asset(&self, name: &str) -> crate::Result<kv::GetOptionsBuilder> {
    //let kv = self.get_assets()?;
    //let name = crate::assets::resolve(name);
    //Ok(kv.get(&name))
    //}

    //pub fn get_assets(&self) -> crate::Result<kv::KvStore> {
    //Ok(self.env.kv(consts::KV_STATIC_CONTENT)?)
    //}

    pub async fn session(&self) -> Result<Option<String>> {
        //let session = match self.req().session() {
        //Some(session) => session,
        //None => return Ok(None),
        //};

        //Ok(Some(self.dangerous()?.verify::<app::User>(&session).await?))
        Ok(Some(String::from("sessionid")))
    }
}

impl Deref for RequestContext {
    type Target = worker::Request;

    fn deref(&self) -> &Self::Target {
        &self.req
    }
}
