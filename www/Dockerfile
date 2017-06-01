FROM python:3.6-alpine

WORKDIR /srv

COPY requirements.txt .
RUN pip install -r requirements.txt

COPY . www
CMD ["gunicorn", \
	 "-w", "8", \
	 "-b", "0.0.0.0:80", \
	 "-n", "lain_www", \
	 "www:app"]
