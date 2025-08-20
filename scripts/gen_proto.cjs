const {execSync} = require('child_process');
const {platform} = require('os');
const {exit} = require('process');

const isWindows = platform() === 'win32';

const pluginPath = isWindows
    ? './node_modules/.bin/protoc-gen-ts_proto.CMD'
    : './node_modules/.bin/protoc-gen-ts_proto';

const command = `protoc --plugin=${pluginPath} --ts_proto_out=src proto/*`;

console.log(`🎯 Platform: ${platform()}`);
console.log(`🚀 Command: ${command}`);

try {
    execSync(command, {stdio: 'inherit'}); // Show real-time output
    console.log('✅ Protocol Buffers generated successfully!');
} catch (error) {
    console.error('❌ protoc execution failed:', error.message);
    exit(1);
}