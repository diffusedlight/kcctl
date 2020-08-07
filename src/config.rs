use std::env

struct config {
  kubeconfig_dir_: /* PathBuf, Path, String containing path to dir for kubeconfigs */,
  active_kubeconfig: /* PathBuf, Path, String containt path to kubeconfig file */,
}

fn get active_kubeconfig_path() -> Result< PathBuf, String > {
  let kubeconfig = env::var("KUBECONFIG")?;
  println!("{:?}", kubeconfig)
}

fn parse_config() -> Result< config, String > {
   /* This should return a struct */ 
   
  }
