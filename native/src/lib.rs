#[macro_use]
extern crate neon;
extern crate neon_serde;
extern crate vultrapi;

use neon::{
    vm::{Call, JsResult},
    js::{JsString, JsValue, Object, error::{Kind, JsError}},
    scope::Scope,
};
use vultrapi::{request::{ServerOptions, VultrRequest}, VultrMgr};


fn servers(call: Call) -> JsResult<JsValue> {
    let scope= call.scope;
    let api_key = scope.global().get(scope, "API_KEY").unwrap()
        .downcast::<JsString>().unwrap().value();
    let vultr_mgr = VultrMgr::with_api_key(&api_key);

    if let Some(server_id) = call.arguments.get(scope, 0) {
        match vultr_mgr.server_by_filter(
            server_id.downcast::<JsString>().unwrap().value().as_str()
        ).retrieve_json() {
            Ok(servers_res) => Ok(neon_serde::to_value(scope, &servers_res).unwrap()),
            Err(e) => JsError::throw(Kind::Error, &format!("{:?}", e)),
        }
    } else {
        match vultr_mgr.servers().retrieve() {
            Ok(servers_res) => Ok(neon_serde::to_value(scope, &servers_res).unwrap()),
            Err(e) => JsError::throw(Kind::Error, &format!("{:?}", e)),
        }
    }
}

fn create_server(call: Call) -> JsResult<JsValue> {
    let scope= call.scope;
    let api_key = scope.global().get(scope, "API_KEY").unwrap()
        .downcast::<JsString>().unwrap().value();
    let vultr_mgr = VultrMgr::with_api_key(&api_key);

    let location = call.arguments.get(scope, 0).unwrap();
    let plan = call.arguments.get(scope, 1).unwrap();
    let snapshot = call.arguments.get(scope, 2).unwrap();
    let hostname = call.arguments.get(scope, 3).unwrap();
    let label = call.arguments.get(scope, 4).unwrap();
    let tag = call.arguments.get(scope, 5).unwrap();

    match vultr_mgr.servers().create(&ServerOptions {
        dc_id: &location.downcast::<JsString>().unwrap().value(),
        vps_plan_id: &plan.downcast::<JsString>().unwrap().value(),
        os_id: "164",
        snapshot_id: Some(&snapshot.downcast::<JsString>().unwrap().value()),
        hostname: Some(&hostname.downcast::<JsString>().unwrap().value()),
        label: Some(&label.downcast::<JsString>().unwrap().value()),
        tag: Some(&tag.downcast::<JsString>().unwrap().value()),
        enable_private_network: Some(true),
    }).retrieve() {
        Ok(server_res) => Ok(neon_serde::to_value(scope, &server_res).unwrap()),
        Err(e) => JsError::throw(Kind::Error, &format!("{:?}", e)),
    }
}

fn destroy_server(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let api_key = scope.global().get(scope, "API_KEY").unwrap()
        .downcast::<JsString>().unwrap().value();
    let vultr_mgr = VultrMgr::with_api_key(&api_key);

    let server_id = call.arguments.get(scope, 0).unwrap();

    match vultr_mgr.servers().destroy(
        &server_id.downcast::<JsString>().unwrap().value()
    ).retrieve() {
        Ok(server_res) => Ok(neon_serde::to_value(scope, &server_res).unwrap()),
        Err(e) => JsError::throw(Kind::Error, &format!("{:?}", e)),
    }
}

fn label_set(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let api_key = scope.global().get(scope, "API_KEY").unwrap()
        .downcast::<JsString>().unwrap().value();
    let vultr_mgr = VultrMgr::with_api_key(&api_key);

    let server_id = call.arguments.get(scope, 0).unwrap();
    let label = call.arguments.get(scope, 1).unwrap();

    match vultr_mgr.servers().label_set(
        &server_id.downcast::<JsString>().unwrap().value(),
        &label.downcast::<JsString>().unwrap().value(),
    ).retrieve() {
        Ok(server_res) => Ok(neon_serde::to_value(scope, &server_res).unwrap()),
        Err(e) => JsError::throw(Kind::Error, &format!("{:?}", e)),
    }
}

fn reboot_server(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let api_key = scope.global().get(scope, "API_KEY").unwrap()
        .downcast::<JsString>().unwrap().value();
    let vultr_mgr = VultrMgr::with_api_key(&api_key);

    let server_id = call.arguments.get(scope, 0).unwrap();

    match vultr_mgr.servers().reboot(
        &server_id.downcast::<JsString>().unwrap().value()
    ).retrieve() {
        Ok(server_res) => Ok(neon_serde::to_value(scope, &server_res).unwrap()),
        Err(e) => JsError::throw(Kind::Error, &format!("{:?}", e)),
    }
}

fn start_server(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let api_key = scope.global().get(scope, "API_KEY").unwrap()
        .downcast::<JsString>().unwrap().value();
    let vultr_mgr = VultrMgr::with_api_key(&api_key);

    let server_id = call.arguments.get(scope, 0).unwrap();

    match vultr_mgr.servers().start(
        &server_id.downcast::<JsString>().unwrap().value()
    ).retrieve() {
        Ok(server_res) => Ok(neon_serde::to_value(scope, &server_res).unwrap()),
        Err(e) => JsError::throw(Kind::Error, &format!("{:?}", e)),
    }
}

fn upgrade_server_plan(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let api_key = scope.global().get(scope, "API_KEY").unwrap()
        .downcast::<JsString>().unwrap().value();
    let vultr_mgr = VultrMgr::with_api_key(&api_key);

    let server_id = call.arguments.get(scope, 0).unwrap();
    let plan_id = call.arguments.get(scope, 1).unwrap();

    match vultr_mgr.servers().upgrade_plan(
        &server_id.downcast::<JsString>().unwrap().value(),
        &plan_id.downcast::<JsString>().unwrap().value(),
    ).retrieve() {
        Ok(server_res) => Ok(neon_serde::to_value(scope, &server_res).unwrap()),
        Err(e) => JsError::throw(Kind::Error, &format!("{:?}", e)),
    }
}

fn upgrade_server_plan_list(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let api_key = scope.global().get(scope, "API_KEY").unwrap()
        .downcast::<JsString>().unwrap().value();
    let vultr_mgr = VultrMgr::with_api_key(&api_key);

    let server_id = call.arguments.get(scope, 0).unwrap();

    match vultr_mgr.servers().upgrade_plan_list(
        &server_id.downcast::<JsString>().unwrap().value(),
    ).retrieve() {
        Ok(server_res) => Ok(neon_serde::to_value(scope, &server_res).unwrap()),
        Err(e) => JsError::throw(Kind::Error, &format!("{:?}", e)),
    }
}

pub fn oses(call: Call) -> JsResult<JsValue> {
    let scope = call.scope;
    let api_key = scope.global().get(scope, "API_KEY").unwrap()
        .downcast::<JsString>().unwrap().value();
    let vultr_mgr = VultrMgr::with_api_key(&api_key);

    match vultr_mgr.operating_systems().retrieve() {
        Ok(oses_res) => Ok(neon_serde::to_value(scope, &oses_res).unwrap()),
        Err(e) => JsError::throw(Kind::Error, &format!("{:?}", e)),
    }
}

fn snapshots(call: Call) -> JsResult<JsValue> {
    let scope= call.scope;
    let api_key = scope.global().get(scope, "API_KEY").unwrap()
        .downcast::<JsString>().unwrap().value();
    let vultr_mgr = VultrMgr::with_api_key(&api_key);

    match vultr_mgr.snapshots().retrieve() {
        Ok(snapshots_res) => Ok(neon_serde::to_value(scope, &snapshots_res).unwrap()),
        Err(e) => JsError::throw(Kind::Error, &format!("{:?}", e)),
    }
}

fn regions(call: Call) -> JsResult<JsValue> {
    let scope= call.scope;
    let api_key = scope.global().get(scope, "API_KEY").unwrap()
        .downcast::<JsString>().unwrap().value();
    let vultr_mgr = VultrMgr::with_api_key(&api_key);

    match vultr_mgr.regions().retrieve() {
        Ok(regions_res) => Ok(neon_serde::to_value(scope, &regions_res).unwrap()),
        Err(e) => JsError::throw(Kind::Error, &format!("{:?}", e)),
    }
}

fn plans(call: Call) -> JsResult<JsValue> {
    let scope= call.scope;
    let api_key = scope.global().get(scope, "API_KEY").unwrap()
        .downcast::<JsString>().unwrap().value();
    let vultr_mgr = VultrMgr::with_api_key(&api_key);

    match vultr_mgr.plans().retrieve() {
        Ok(plans_res) => Ok(neon_serde::to_value(scope, &plans_res).unwrap()),
        Err(e) => JsError::throw(Kind::Error, &format!("{:?}", e)),
    }
}

register_module!(m, {
    m.export("createServer", create_server)?;
    m.export("destroyServer", destroy_server)?;
    m.export("labelSet", label_set)?;
    m.export("oses", oses)?;
    m.export("plans", plans)?;
    m.export("rebootServer", reboot_server)?;
    m.export("regions", regions)?;
    m.export("servers", servers)?;
    m.export("snapshots", snapshots)?;
    m.export("startServer", start_server)?;
    m.export("upgradeServerPlan", upgrade_server_plan)?;
    m.export("upgradeServerPlanList", upgrade_server_plan_list)?;
    Ok(())
});
