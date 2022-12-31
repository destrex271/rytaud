# rytaud

rytaud is a free to use API to convert youtube videos into mp3 format. As of now it is not hosted as it is still in its initial stages of developement but you can try out the [docker image](https://github.com/destrex271/rytaud/pkgs/container/rytaud) available as a github package.

You can also clone this repository and run the following command:

```
docker-compose up --build --remove-orphans
```
This will start the container on your local machine. The API will pe available at 0.0.0.0:800.

## Endpoints

```
/download_audio
```
The request body structure is as follows:
```
{
    key: "file name goes here",
    url: "url of youtube video goes here"
}
```

The response consists of 
```
{
    file: "/audio/filename.mp3"
}
```
The file can be accessed as 0.0.0.0:8000/audio/filename.mp3

Errors are as follows:
```
{
    error: "Error message"
}
```

## Contributing
To contribute to this project
- Either open an issue or solve an existing issue.
- Fork this repository and work on seperate branches.
- The PRs should be brief and sufficient to explain the feature/bug fix that you have added.