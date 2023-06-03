deployConfig="${TURNIN_FRONTEND_CONFIG_DEPLOY:-config_deploy.json}"

mkdir -p ./config

if [ -f "$deployConfig" ];
    then 
        echo "Using config file ${deployConfig}"
        cp "${deployConfig}" config/config.json
    exit 0
fi

developmentConfig="config_local.json"

echo "Using development config file ${developmentConfig}"
cp "${developmentConfig}" config/config.json




