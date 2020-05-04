.PHONY: build-docker
build-docker:
	docker run --rm -v `pwd`/src:/work vvakame/review:4.0 /bin/sh -c "cd /work && review-pdfmaker config.yml"

