#!/usr/bin/env python

import os
import os.path

from jinja2 import Environment, PackageLoader, select_autoescape

env = Environment(
    loader=PackageLoader("generate_solvers"), autoescape=select_autoescape()
)


def generate_day_solver(day, year_module_directory):
    day_solver_template = env.get_template("day_solver.rs.jinja")
    rendered_day_solver_source = day_solver_template.render(day=str(day).zfill(2))
    day_solver_source = os.path.join(
        year_module_directory, f"day{str(day).zfill(2)}_solver.rs"
    )
    with open(day_solver_source, "w") as day_solver_file:
        day_solver_file.write(rendered_day_solver_source)


def generate_day_solvers(year_module_directory):
    days = range(1, 26)
    for day in days:
        generate_day_solver(day, year_module_directory)


def generate_year_solvers(year, src_directory):
    year_solver_source = os.path.join(src_directory, f"aoc_{year}.rs")
    if not os.path.exists(year_solver_source):
        year_solver_template = env.get_template("aoc_year.rs.jinja")
        rendered_year_solver = year_solver_template.render()
        with open(year_solver_source, "w") as year_solver_file:
            year_solver_file.write(rendered_year_solver)
    year_module_directory = os.path.join(src_directory, f"aoc_{year}")
    if not os.path.exists(year_module_directory):
        os.makedirs(year_module_directory)
        print(f"Created src directory: {year_module_directory}")
        generate_day_solvers(year_module_directory)


def generate_years_solvers(years, src_directory):
    for year in years:
        generate_year_solvers(year, src_directory)


def generate_main(years, src_directory):
    main_source_template = env.get_template("main.rs.jinja")
    rendered_main_source = main_source_template.render(years=years)
    main_source = os.path.join(src_directory, "main.rs")
    with open(main_source, "w") as ms:
        ms.write(rendered_main_source)


def main() -> None:
    src_directory = os.path.join(os.path.curdir, "src")
    if not os.path.exists(src_directory):
        os.makedirs(src_directory)
        print(f"Created src directory: {src_directory}")
    first_year = 2015
    last_year = 2023
    years = range(first_year, last_year + 1)
    generate_main(years, src_directory)
    generate_years_solvers(years, src_directory)


if __name__ == "__main__":
    main()
