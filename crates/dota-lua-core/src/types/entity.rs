use super::vector::Vector;

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CEntityInstance;

impl CEntityInstance {
    pub fn ConnectOutput(&self, arg1: String, arg2: String) { unimplemented!() }
    pub fn Destroy(&self) { unimplemented!() }
    pub fn DisconnectOutput(&self, arg1: String, arg2: String) { unimplemented!() }
    pub fn DisconnectRedirectedOutput(&self, arg1: String, arg2: String, arg3: crate::types::Table) { unimplemented!() }
    pub fn FireOutput(&self, arg1: String, arg2: crate::types::Table, arg3: crate::types::Table, arg4: crate::types::Any, arg5: f64) { unimplemented!() }
    pub fn GetClassname(&self) -> String { unimplemented!() }
    pub fn GetDebugName(&self) -> String { unimplemented!() }
    pub fn GetEntityHandle(&self) -> CBaseEntity { unimplemented!() }
    pub fn GetEntityIndex(&self) -> i32 { unimplemented!() }
    pub fn GetIntAttr(&self, arg1: String) -> i32 { unimplemented!() }
    pub fn GetName(&self) -> String { unimplemented!() }
    pub fn GetOrCreatePrivateScriptScope(&self) -> crate::types::Any { unimplemented!() }
    pub fn GetOrCreatePublicScriptScope(&self) -> crate::types::Any { unimplemented!() }
    pub fn GetPrivateScriptScope(&self) -> crate::types::Any { unimplemented!() }
    pub fn GetPublicScriptScope(&self) -> crate::types::Any { unimplemented!() }
    pub fn RedirectOutput(&self, arg1: String, arg2: String, arg3: crate::types::Table) { unimplemented!() }
    pub fn RemoveSelf(&self) { unimplemented!() }
    pub fn SetIntAttr(&self, arg1: String, arg2: i32) { unimplemented!() }
    pub fn entindex(&self) -> i32 { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CBaseEntity;

impl CBaseEntity {
    pub fn AddEffects(&self, n_flags: i32) { unimplemented!() }
    pub fn ApplyAbsVelocityImpulse(&self, vec_impulse: Vector) { unimplemented!() }
    pub fn ApplyLocalAngularVelocityImpulse(&self, ang_impulse: Vector) { unimplemented!() }
    pub fn Attribute_GetFloatValue(&self, p_name: String, fl_default: f64) -> f64 { unimplemented!() }
    pub fn Attribute_GetIntValue(&self, p_name: String, n_default: i32) -> i32 { unimplemented!() }
    pub fn Attribute_SetFloatValue(&self, p_name: String, fl_value: f64) { unimplemented!() }
    pub fn Attribute_SetIntValue(&self, p_name: String, n_value: i32) { unimplemented!() }
    pub fn DeleteAttribute(&self, p_name: String) { unimplemented!() }
    pub fn EmitSound(&self, soundname: String) { unimplemented!() }
    pub fn EmitSoundParams(&self, soundname: String, n_pitch: f64, fl_volume: f64, fl_delay: f64) { unimplemented!() }
    pub fn EyeAngles(&self) -> crate::types::QAngle { unimplemented!() }
    pub fn EyePosition(&self) -> Vector { unimplemented!() }
    pub fn FirstMoveChild(&self) -> CBaseEntity { unimplemented!() }
    pub fn FollowEntity(&self, h_ent: CBaseEntity, b_bone_merge: bool) { unimplemented!() }
    pub fn GatherCriteria(&self, h_result: crate::types::Table) { unimplemented!() }
    pub fn GetAbsOrigin(&self) -> Vector { unimplemented!() }
    pub fn GetAbsScale(&self) -> f64 { unimplemented!() }
    pub fn GetAngles(&self) -> crate::types::QAngle { unimplemented!() }
    pub fn GetAnglesAsVector(&self) -> Vector { unimplemented!() }
    pub fn GetAngularVelocity(&self) -> Vector { unimplemented!() }
    pub fn GetBaseVelocity(&self) -> Vector { unimplemented!() }
    pub fn GetBoundingMaxs(&self) -> Vector { unimplemented!() }
    pub fn GetBoundingMins(&self) -> Vector { unimplemented!() }
    pub fn GetBounds(&self) -> crate::types::Any { unimplemented!() }
    pub fn GetCenter(&self) -> Vector { unimplemented!() }
    pub fn GetChildren(&self) -> Vec<CBaseEntity> { unimplemented!() }
    pub fn GetContext(&self, name: String) -> crate::types::Any { unimplemented!() }
    pub fn GetForwardVector(&self) -> Vector { unimplemented!() }
    pub fn GetHealth(&self) -> i32 { unimplemented!() }
    pub fn GetLocalAngles(&self) -> crate::types::QAngle { unimplemented!() }
    pub fn GetLocalAngularVelocity(&self) -> crate::types::QAngle { unimplemented!() }
    pub fn GetLocalOrigin(&self) -> Vector { unimplemented!() }
    pub fn GetLocalScale(&self) -> f64 { unimplemented!() }
    pub fn GetLocalVelocity(&self) -> Vector { unimplemented!() }
    pub fn GetMass(&self) -> f64 { unimplemented!() }
    pub fn GetMaxHealth(&self) -> i32 { unimplemented!() }
    pub fn GetModelName(&self) -> String { unimplemented!() }
    pub fn GetMoveParent(&self) -> CBaseEntity { unimplemented!() }
    pub fn GetOrigin(&self) -> Vector { unimplemented!() }
    pub fn GetOwner(&self) -> CBaseEntity { unimplemented!() }
    pub fn GetOwnerEntity(&self) -> CBaseEntity { unimplemented!() }
    pub fn GetRightVector(&self) -> Vector { unimplemented!() }
    pub fn GetRootMoveParent(&self) -> CBaseEntity { unimplemented!() }
    pub fn GetSoundDuration(&self, soundname: String, actormodel: String) -> f64 { unimplemented!() }
    pub fn GetTeam(&self) -> i32 { unimplemented!() }
    pub fn GetTeamNumber(&self) -> i32 { unimplemented!() }
    pub fn GetUpVector(&self) -> Vector { unimplemented!() }
    pub fn GetVelocity(&self) -> Vector { unimplemented!() }
    pub fn HasAttribute(&self, p_name: String) -> bool { unimplemented!() }
    pub fn IsAlive(&self) -> bool { unimplemented!() }
    pub fn Kill(&self) { unimplemented!() }
    pub fn NextMovePeer(&self) -> CBaseEntity { unimplemented!() }
    pub fn OverrideFriction(&self, duration: f64, friction: f64) { unimplemented!() }
    pub fn PrecacheScriptSound(&self, soundname: String) { unimplemented!() }
    pub fn RemoveEffects(&self, n_flags: i32) { unimplemented!() }
    pub fn SetAbsAngles(&self, f_pitch: f64, f_yaw: f64, f_roll: f64) { unimplemented!() }
    pub fn SetAbsOrigin(&self, origin: Vector) { unimplemented!() }
    pub fn SetAbsScale(&self, fl_scale: f64) { unimplemented!() }
    pub fn SetAngles(&self, f_pitch: f64, f_yaw: f64, f_roll: f64) { unimplemented!() }
    pub fn SetAngularVelocity(&self, pitch_vel: f64, yaw_vel: f64, roll_vel: f64) { unimplemented!() }
    pub fn SetConstraint(&self, v_pos: Vector) { unimplemented!() }
    pub fn SetContext(&self, p_name: String, p_value: String, duration: f64) { unimplemented!() }
    pub fn SetContextNum(&self, p_name: String, f_value: f64, duration: f64) { unimplemented!() }
    pub fn SetContextThink(&self, psz_context_name: String, h_think_func: crate::types::Function, fl_interval: f64) { unimplemented!() }
    pub fn SetEntityName(&self, p_name: String) { unimplemented!() }
    pub fn SetForwardVector(&self, v: Vector) { unimplemented!() }
    pub fn SetFriction(&self, fl_friction: f64) { unimplemented!() }
    pub fn SetGravity(&self, fl_gravity: f64) { unimplemented!() }
    pub fn SetHealth(&self, n_health: i32) { unimplemented!() }
    pub fn SetLocalAngles(&self, f_pitch: f64, f_yaw: f64, f_roll: f64) { unimplemented!() }
    pub fn SetLocalOrigin(&self, origin: Vector) { unimplemented!() }
    pub fn SetLocalScale(&self, fl_scale: f64) { unimplemented!() }
    pub fn SetMass(&self, fl_mass: f64) { unimplemented!() }
    pub fn SetMaxHealth(&self, amt: i32) { unimplemented!() }
    pub fn SetOrigin(&self, v: Vector) { unimplemented!() }
    pub fn SetOwner(&self, p_owner: CBaseEntity) { unimplemented!() }
    pub fn SetParent(&self, h_parent: CBaseEntity, p_attachmentname: String) { unimplemented!() }
    pub fn SetTeam(&self, i_team_num: i32) { unimplemented!() }
    pub fn SetVelocity(&self, vec_velocity: Vector) { unimplemented!() }
    pub fn StopSound(&self, soundname: String) { unimplemented!() }
    pub fn TakeDamage(&self, h_info: crate::types::Table) -> i32 { unimplemented!() }
    pub fn Trigger(&self) { unimplemented!() }
    pub fn ValidatePrivateScriptScope(&self) { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CBaseModelEntity;

impl CBaseModelEntity {
    pub fn GetMaterialGroupHash(&self) -> i32 { unimplemented!() }
    pub fn GetMaterialGroupMask(&self) -> i64 { unimplemented!() }
    pub fn GetRenderAlpha(&self) -> i32 { unimplemented!() }
    pub fn GetRenderColor(&self) -> Vector { unimplemented!() }
    pub fn SetLightGroup(&self, p_light_group: String) { unimplemented!() }
    pub fn SetMaterialGroup(&self, p_material_group: String) { unimplemented!() }
    pub fn SetMaterialGroupHash(&self, n_hash: i32) { unimplemented!() }
    pub fn SetMaterialGroupMask(&self, n_mesh_group_mask: i64) { unimplemented!() }
    pub fn SetModel(&self, p_model_name: String) { unimplemented!() }
    pub fn SetRenderAlpha(&self, n_alpha: i32) { unimplemented!() }
    pub fn SetRenderColor(&self, r: i32, g: i32, b: i32) { unimplemented!() }
    pub fn SetRenderMode(&self, n_mode: i32) { unimplemented!() }
    pub fn SetSingleMeshGroup(&self, p_mesh_group_name: String) { unimplemented!() }
    pub fn SetSize(&self, mins: Vector, maxs: Vector) { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CBaseAnimating;

impl CBaseAnimating {
    pub fn ActiveSequenceDuration(&self) -> f64 { unimplemented!() }
    pub fn GetAttachmentAngles(&self, i_attachment: i32) -> Vector { unimplemented!() }
    pub fn GetAttachmentOrigin(&self, i_attachment: i32) -> Vector { unimplemented!() }
    pub fn GetGraphParameter(&self, psz_param: String) -> crate::types::Any { unimplemented!() }
    pub fn GetModelScale(&self) -> f64 { unimplemented!() }
    pub fn GetSequence(&self) -> String { unimplemented!() }
    pub fn IsSequenceFinished(&self) -> bool { unimplemented!() }
    pub fn ResetSequence(&self, p_sequence_name: String) { unimplemented!() }
    pub fn ScriptLookupAttachment(&self, p_attachment_name: String) -> i32 { unimplemented!() }
    pub fn SequenceDuration(&self, p_sequence_name: String) -> f64 { unimplemented!() }
    pub fn SetBodygroup(&self, i_group: i32, i_value: i32) { unimplemented!() }
    pub fn SetBodygroupByName(&self, p_name: String, i_value: i32) { unimplemented!() }
    pub fn SetGraphLookDirection(&self, v_value: Vector) { unimplemented!() }
    pub fn SetGraphParameter(&self, psz_param: String, sv_arg: crate::types::Any) { unimplemented!() }
    pub fn SetGraphParameterBool(&self, sz_name: String, b_value: bool) { unimplemented!() }
    pub fn SetGraphParameterEnum(&self, sz_name: String, n_value: i32) { unimplemented!() }
    pub fn SetGraphParameterFloat(&self, sz_name: String, fl_value: f64) { unimplemented!() }
    pub fn SetGraphParameterInt(&self, sz_name: String, n_value: i32) { unimplemented!() }
    pub fn SetGraphParameterVector(&self, sz_name: String, v_value: Vector) { unimplemented!() }
    pub fn SetModelScale(&self, fl_scale: f64) { unimplemented!() }
    pub fn SetPoseParameter(&self, sz_name: String, f_value: f64) -> i32 { unimplemented!() }
    pub fn SetProceduralIKTarget(&self, p_chain_name: String, p_target_name: String, v_target_position: Vector, q_target_rotation: crate::types::QAngle) -> bool { unimplemented!() }
    pub fn SetProceduralIKTargetWeight(&self, p_chain_name: String, p_target_name: String, fl_weight: f64) -> bool { unimplemented!() }
    pub fn SetSequence(&self, p_sequence_name: String) { unimplemented!() }
    pub fn SetSkin(&self, i_skin: i32) { unimplemented!() }
    pub fn StopAnimation(&self) { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CBaseFlex;

impl CBaseFlex {
    pub fn GetCurrentScene(&self) -> super::misc::CSceneEntity { unimplemented!() }
    pub fn GetSceneByIndex(&self, index: i32) -> super::misc::CSceneEntity { unimplemented!() }
    pub fn ScriptPlayScene(&self, psz_scene: String, fl_delay: f64) -> i32 { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CBaseCombatCharacter;

impl CBaseCombatCharacter {
    pub fn GetEquippedWeapons(&self) -> crate::types::Any { unimplemented!() }
    pub fn GetFaction(&self) -> i32 { unimplemented!() }
    pub fn GetWeaponCount(&self) -> i32 { unimplemented!() }
    pub fn ShootPosition(&self, n_hand: i32) -> Vector { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CBasePlayer;

impl CBasePlayer {
    pub fn AreChaperoneBoundsVisible(&self) -> bool { unimplemented!() }
    pub fn GetHMDAnchor(&self) -> crate::types::Any { unimplemented!() }
    pub fn GetHMDAvatar(&self) -> crate::types::Any { unimplemented!() }
    pub fn GetPlayArea(&self, n_point: i32) -> Vector { unimplemented!() }
    pub fn GetUserID(&self) -> i32 { unimplemented!() }
    pub fn GetVRControllerType(&self) -> crate::types::Any { unimplemented!() }
    pub fn IsNoclipping(&self) -> bool { unimplemented!() }
    pub fn IsUsePressed(&self) -> bool { unimplemented!() }
    pub fn IsVRControllerButtonPressed(&self, n_button: i32) -> bool { unimplemented!() }
    pub fn IsVRDashboardShowing(&self) -> bool { unimplemented!() }
    pub fn Quit(&self) { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CBaseTrigger;

impl CBaseTrigger {
    pub fn Disable(&self) { unimplemented!() }
    pub fn Enable(&self) { unimplemented!() }
    pub fn IsTouching(&self, h_ent: CBaseEntity) -> bool { unimplemented!() }
}

#[allow(non_snake_case)]
#[allow(unused)]
pub struct CBodyComponent;

impl CBodyComponent {
    pub fn AddImpulseAtPosition(&self, arg1: Vector, arg2: Vector) { unimplemented!() }
    pub fn AddVelocity(&self, arg1: Vector, arg2: Vector) { unimplemented!() }
    pub fn DetachFromParent(&self) { unimplemented!() }
    pub fn GetSequence(&self) -> crate::types::Any { unimplemented!() }
    pub fn IsAttachedToParent(&self) -> bool { unimplemented!() }
    pub fn LookupSequence(&self, arg1: String) -> crate::types::Any { unimplemented!() }
    pub fn SequenceDuration(&self, arg1: String) -> f64 { unimplemented!() }
    pub fn SetAngularVelocity(&self, arg1: Vector) { unimplemented!() }
    pub fn SetAnimation(&self, arg1: String) { unimplemented!() }
    pub fn SetBodyGroup(&self, arg1: String) { unimplemented!() }
    pub fn SetMaterialGroup(&self, arg1: String) { unimplemented!() }
    pub fn SetVelocity(&self, arg1: Vector) { unimplemented!() }
}
