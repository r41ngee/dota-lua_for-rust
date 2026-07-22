use super::entity::CBaseEntity;
use super::vector::Vector;

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CScriptHTTPResponse;

impl CScriptHTTPResponse {
    pub fn new() -> Self { unimplemented!() }
    pub fn Body(&self) -> String { unimplemented!() }
    pub fn Request(&self) -> CScriptHTTPRequest { unimplemented!() }
    pub fn StatusCode(&self) -> i32 { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CScriptHTTPRequest;

#[allow(non_snake_case)]
#[allow(unused)]
impl CScriptHTTPRequest {
    pub fn CreateHTTPRequest(method: String, url: String) -> CScriptHTTPRequest { unimplemented!() }
    pub fn CreateHTTPRequestScriptVM(method: String, url: String) -> CScriptHTTPRequest { unimplemented!() }
    pub fn SetHTTPRequestAbsoluteTimeoutMS(&self, absolute_timeout_ms: i32) { unimplemented!() }
    pub fn SetHTTPRequestContentLength(&self, content_length: i32) { unimplemented!() }
    pub fn SetHTTPRequestCookie(&self, cookie_name: String, cookie_value: String) { unimplemented!() }
    pub fn SetHTTPRequestGetOrPostParameter(&self, param_name: String, param_value: String) { unimplemented!() }
    pub fn SetHTTPRequestHeader(&self, header_name: String, header_value: String) { unimplemented!() }
    pub fn SetHTTPRequestNetworkActivityTimeout(&self, network_activity_timeout: i32) { unimplemented!() }
    pub fn SetHTTPRequestRawPostBody(&self, body: String) { unimplemented!() }
    pub fn Send(&self, completion_callback: crate::types::Function) -> CScriptHTTPResponse { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CScriptPrecacheContext;

impl CScriptPrecacheContext {
    pub fn AddResource(&self, arg1: String) { unimplemented!() }
    pub fn GetValue(&self, arg1: String) -> crate::types::Any { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CScriptUniformRandomStream;

impl CScriptUniformRandomStream {
    pub fn RandomFloat(&self, fl_min: f64, fl_max: f64) -> f64 { unimplemented!() }
    pub fn RandomInt(&self, n_min: i32, n_max: i32) -> i32 { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CSceneEntity;

impl CSceneEntity {
    pub fn AddBroadcastTeamTarget(&self, arg1: i32) { unimplemented!() }
    pub fn Cancel(&self) { unimplemented!() }
    pub fn EstimateLength(&self) -> f64 { unimplemented!() }
    pub fn FindCamera(&self) -> crate::types::Any { unimplemented!() }
    pub fn FindNamedEntity(&self, arg1: String) -> CBaseEntity { unimplemented!() }
    pub fn IsPaused(&self) -> bool { unimplemented!() }
    pub fn IsPlayingBack(&self) -> bool { unimplemented!() }
    pub fn LoadSceneFromString(&self, arg1: String, arg2: String) -> bool { unimplemented!() }
    pub fn RemoveBroadcastTeamTarget(&self, arg1: i32) { unimplemented!() }
    pub fn Start(&self, arg1: CBaseEntity) { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CEnvEntityMaker;

impl CEnvEntityMaker {
    pub fn SpawnEntity(&self) { unimplemented!() }
    pub fn SpawnEntityAtEntityOrigin(&self, h_entity: CBaseEntity) { unimplemented!() }
    pub fn SpawnEntityAtLocation(&self, vec_alternate_origin: Vector, vec_alternate_angles: crate::types::QAngle) { unimplemented!() }
    pub fn SpawnEntityAtNamedEntityOrigin(&self, psz_name: String) { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CBaseItemPhysical;

impl CBaseItemPhysical {
    pub fn GetContainedItem(&self) -> super::item::CBaseItem { unimplemented!() }
    pub fn GetCreationTime(&self) -> f64 { unimplemented!() }
    pub fn SetContainedItem(&self, h_item: super::item::CBaseItem) { unimplemented!() }
}
