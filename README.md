# Chess Framework For Engines and Data Science

While this project origionally started as a tauri native app project, I have gone ahead and migrated to a full stack web app. The goal of this project is to create a chess framework that can be used for both engines and data science.

I did this because I wanted a larger user base where I could gather game data and better train the engines, and I have a home server now so I could host and do this easily

It is not quite complete yet, but I plan to have this all selfhosted and with an account system for people to track their games and progress. I also plan to have a full API for the engines to use, and a full database of games and positions for data science.

---

## Running the Project

This project is all docker based, so you will need to have docker installed. You can find the instructions for installing docker [here](https://docs.docker.com/get-docker/).

To run the full app:

```bash
docker-compose up -d
```
