use crate::{context::Context, process::process_platforms};

pub async fn process_exports(ctx: &mut Context, exports: &Vec<flake_info::data::Export>) {
    for export in exports {
        match &export.item {
            flake_info::data::Derivation::Package {
                package_platforms, ..
            } => {
                let p = process_platforms(
                    &mut ctx.pg_conn,
                    &mut ctx.platforms_cache,
                    package_platforms,
                );
                println!("{:?}", package_platforms);
                println!("{:?}", p);
            }
            _ => todo!(),
        };

        println!("{:?}", export);
    }
}
