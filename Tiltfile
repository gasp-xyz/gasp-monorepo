aggregator = helm(
    'ops/helm-charts/rollup-node',
    name='avs-aggregator',
    set=[
        'image.repository=mangatasolutions/avs-aggregator',
        'image.tag=c7da7452e46aa08b5dc33d0c3d23c18c8319f9c6',
        'environment=local'
    ])
anvil = helm('ops/helm-charts/anvil-testnet',  name='anvil')
    
k8s_yaml([anvil, aggregator])

k8s_resource(workload='anvil', port_forwards=8545, objects=['anvil-testnet-configmap'])
k8s_resource(workload='avs-aggregator', port_forwards=8090, objects=['avs-aggregator-secret'])