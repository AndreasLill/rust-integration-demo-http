import http from 'k6/http';

export const options = {
  vus: 200,
  duration: '1s',
};

export default function () {
  const url = 'http://127.0.0.1:8080/xml';

  const payload = JSON.stringify({
    name: "Andreas",
    id: "123456",
  });

  const params = {
    headers: {
      'Content-Type': 'application/json',
    },
  };

  http.post(url, payload, params);
}
