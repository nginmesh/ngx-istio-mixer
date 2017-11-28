MODULE_NAME=ngx_http_istio_mixer_module
MODULE_PROJ_NAME=ngx-http-istio-mixer
NGX_DEBUG="--with-debug"
VERSION=0.2.12-RC2
REPO=ngx-istio-mixer
USER=nginmesh


include nginx.mk


clean:
	cargo clean
	rm -f ${MIXER_CRATE}/src/grpc/attributes.rs
	rm -f ${MIXER_CRATE}/src/grpc/status.rs
	rm -f ${MIXER_CRATE}/src/grpc/check.rs
	rm -f ${MIXER_CRATE}/src/grpc/quota.rs
	rm -f ${MIXER_CRATE}/src/grpc/report.rs
	rm -f ${MIXER_CRATE}/src/grpc/service_grpc.rs
	rm -f module/*.so
	rm -rf dockercontext


super_clean: clean
	rm -rf nginx/*


report:
	cargo build --bin report_client

test:
	cargo test -- --nocapture


release:	create_release upload_release

create_release:
	github-release release \
    --user ${USER} \
    --repo ${REPO} \
    --tag ${VERSION} \
    --name "${VERSION}" \
    --pre-release


upload_release:
	github-release upload \
    	--user ${USER} \
    	--repo ${REPO} \
    	--tag ${VERSION} \
    	--name "${MODULE_NAME}.so" \
    	--file module/release/${MODULE_NAME}.so


delete_release:
	github-release delete \
    --user ${USER} \
    --repo ${REPO} \
    --tag ${VERSION}