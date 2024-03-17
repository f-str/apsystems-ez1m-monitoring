# Simple Dashboard for APSystems EZ1-M Microinverter

```shell
docker build -t apsystems-dashboard .
```

Run the dashboard with the following command:
```shell
docker run -d -p 80:80 --network host --restart unless-stopped --name apsystems-dashboard apsystems-dashboard
```
The dashboard will be available at http://localhost:80/

In order to have a functional dashboard, you need a running instance of the [rcp](https://github.com/f-str/rcp) service:
```shell
git clone https://github.com/f-str/rcp.git
cd rcp
docker build -t rcp .

docker run -d -p 8080:8080 --network host --restart unless-stopped --name rcp rcp
```

