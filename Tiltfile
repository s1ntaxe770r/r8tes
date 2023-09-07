k8s_yaml(['manifests/role.yaml', 'manifests/controller-manager.yaml'])


api_pod = 'deploy/rates-controller-manager' 
local_resource('kube-logs', serve_cmd='kubectl logs -f -n rates {}'.format(api_pod))

