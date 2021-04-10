package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
	"net/url"
	"os"
	"strings"

	"github.com/gin-contrib/sessions"
	"github.com/gin-contrib/sessions/cookie"
	"github.com/gin-gonic/gin"
	"github.com/joho/godotenv"
)

type User struct {
	Id        int32  `json:"id"`
	Login     string `json:"login"`
	AvatarUrl string `json:"avatar_url"`
}

func index(c *gin.Context) {
	session := sessions.Default(c)
	accessToken := session.Get("access_token")
	if accessToken == nil {
		c.Redirect(http.StatusSeeOther, "/auth")
	} else {
		req, err := http.NewRequest("GET", "https://api.github.com/user", nil)
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{
				"error": err,
			})
		}
		req.Header.Set("authorization", fmt.Sprintf("bearer %s", accessToken))
		client := &http.Client{}
		res, err := client.Do(req)
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{
				"error": err,
			})
		}
		defer res.Body.Close()
		body, err := ioutil.ReadAll(res.Body)
		var user User
		err = json.Unmarshal(body, &user)
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{
				"error": err,
			})
		}
		c.JSON(http.StatusOK, user)
	}
}

func auth(c *gin.Context) {
	clientId := os.Getenv("CLIENT_ID")
	c.Redirect(
		http.StatusSeeOther,
		fmt.Sprintf("https://github.com/login/oauth/authorize?scope=user:email&client_id=%s", clientId),
	)
}

type AccessTokenResponse struct {
	AccessToken string `json:"access_token"`
	TokenType   string `json:"token_type"`
	Scope       string `json:"scope"`
}

func callback(c *gin.Context) {
	form := url.Values{}
	form.Add("client_id", os.Getenv("CLIENT_ID"))
	form.Add("client_secret", os.Getenv("CLIENT_SECRET"))
	form.Add("code", c.Query("code"))
	req, err := http.NewRequest("POST", "https://github.com/login/oauth/access_token", strings.NewReader(form.Encode()))
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"error": err,
		})
	}
	req.Header.Set("accept", "application/json")
	client := &http.Client{}
	res, err := client.Do(req)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"error": err,
		})
	}
	defer res.Body.Close()
	body, err := ioutil.ReadAll(res.Body)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"error": err,
		})
	}
	var token AccessTokenResponse
	err = json.Unmarshal(body, &token)
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"error": err,
		})
	}
	session := sessions.Default(c)
	session.Set("access_token", token.AccessToken)
	session.Save()
	c.Redirect(http.StatusSeeOther, "/")
}

func main() {
	godotenv.Load()
	engine := gin.Default()
	store := cookie.NewStore([]byte("secret"))
	engine.Use(sessions.Sessions("mysession", store))
	engine.GET("/", index)
	engine.GET("/auth", auth)
	engine.GET("/callback", callback)
	engine.Run(":8080")
}
