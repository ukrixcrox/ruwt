# Notes/design
- in the future multiple different templates should be supported
- the config would now hold these different templates and look a little like this: 
```
  .config/ruwt/
              /config.toml  --description, name and path for each template so that custom templates can be easily added
              /templates/   --here are all the templates  
                        /standard/   --standard html page
                        /admin-page/ --admin page
                        /login-page/ --login page
                        /.../        --more boilerplate to come
                      
              /style/       --here are the css files
                    /dark/  --darkmode css files
                    /light/  --lightmode css files
```

the flag `--templates` that lists all the available templates should get the `template_name`, the `template_description` and the `template_path` from `config.toml`
so that custom templates can be easily created and used
