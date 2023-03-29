import { createSnakeCaseProxy, runTests } from 'azle/test';
import { getTests } from 'azle/examples/motoko_examples/hello-world/test/tests';
import { createActor } from './dfx_generated/hello_world';

const helloWorldCanister = createActor('rrkah-fqaaa-aaaaa-aaaaq-cai', {
    agentOptions: {
        host: 'http://127.0.0.1:8000'
    }
});

runTests(getTests(createSnakeCaseProxy(helloWorldCanister)));
