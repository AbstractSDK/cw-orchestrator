# Starship Setup

> See [starship docs](https://starship.cosmology.tech/) for detailed setup instructions.

**Setup**

Do this only once to create the Starship setup

```bash
make setup
```

**Start Infrastructure**

To start starship, use : 
```bash
make install
```

Once all nodes are up (see next section to check status) you are readi to use starship using cw-orchestrator.

**Check status**

To know the status of all the starship pods, use : 

```bash
kubectl get pods
```

Or for a watch version : 
```bash
make watch-pods
```

**Stop Infrastructure**


```bash
make stop
```

Starship will crash when running for around a day. You will need to restart it from time to time to make sure everything is working. To do so, just stop the infrastructure, verify everything is stopped and `make install` it again 

**Delete Infrastructure**

```bash
make clean-kind
```
