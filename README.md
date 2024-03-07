
![rustacean-orig-noshadow](https://github.com/ysooun/rust_api_server/assets/154872496/a7a85aa1-d472-41b8-8049-c67f435678f8)

### 구성
---
1. Rust 언어를 사용해 health check가 가능한 api server 구성했습니다. 
2. mysql 데이터베이스와 연결하여 users 테이블을 만들고 query를 이용하여 테이블 내 모든 유저 정보를 가져오도록 만들었습니다.
3. 연결이 확인 된 후 도커파일을 사용해 이미지 만들었습니다. 이 이미지 파일은 쿠버네티스에서 배포용 이미지로 사용됩니다.

<br>

### Dockerfile
---
1. 컴파일 환경 분리: 컴파일 환경을 분리하여 이미지를 빌드하면 이미지의 크기를 줄일 수 있고 빌드 속도를 높일 수 있습니다.
2. 런타임 환경 최적화: 이전 단계에서 빌드 될 때 컴파일에 필요한 도구나 라이브러리를 제외하고 이미지를 빌드하기 때문에 바이너리만 포합됩니다. 최종 이미지의 크기는 줄어들고 보안도 좋아집니다.

<br>

### 트러블 슈팅
---
정상적으로 이미지가 빌드 됬으나 배포가 되지 않는 현상
* 에러
  Error: Os { code: 99, kind: AddrNotAvailable, message: "Address not available" }

* 원인
  docker는 기본적으로 호스트 시스템의 OS와 아키텍쳐를 대상으로 이미지를 빌드합니다. 빌드한 결과를 확인해 보면 linux/arm64/v8로 빌드가 된 것을 확인 할 수 있습니다. 하지만 이렇게 빌드 했을 경우 리눅스 환경에서 이미지 배포가 되지 않습니다. 이를 해결하기 위해 docker 이미지로 빌드 할때 platform 부분을 지정해서 이미지를 빌드합니다.

* 해결
  docker buildx build --platform=linux/amd64 -t renum/rust:v0.0.11 . 
  이런 형태로 이미지를 빌드하면 리눅스 환경에서 정상적으로 이미지를 사용할 수 있다.


도커 허브
https://hub.docker.com/repository/docker/renum/rust/general
