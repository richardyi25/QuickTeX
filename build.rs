fn main(){
	std::env::set_var("LALRPOP_LANE_TABLE", "disabled");
	lalrpop::process_root().unwrap();
}