# Bow Task Runner

Similar in idea as _Celery_ in that its aim is to run tasks in parallel.
The name comes from its use of _Apache Arrow_ to setup individual tasks with shared memory.
The core is a mechanism for which to supervise and orchestrate tasks.
Tasks themselves receive details about their execution state either by updating it their selves or from the supervisor.
