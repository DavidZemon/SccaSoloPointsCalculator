import { ChangeEvent, Component, ComponentPropsWithoutRef } from 'react';
import 'bootswatch/dist/darkly/bootstrap.css';
import { Button, Col, Container, Form, Row, Table } from 'react-bootstrap';
import { ClassResultsProcessor } from './services';
import {
  ClassCategoryResults,
  ClassResults,
  EventResults,
  LapTime,
} from './models';

interface AppState {
  eventResultsFile?: File;
  results?: EventResults;
}

class App extends Component<ComponentPropsWithoutRef<any>, AppState> {
  private readonly processor = new ClassResultsProcessor();
  constructor(props: Readonly<ComponentPropsWithoutRef<any>>) {
    super(props);
    this.state = {};
  }

  render() {
    return (
      <Container>
        <Row>
          <Col>
            <h1>SCCA Solo Points Calculator</h1>
          </Col>
        </Row>
        <Row>
          <Col>
            {this.state.eventResultsFile ? (
              <p>
                Ready to process <code>{this.state.eventResultsFile.name}</code>
              </p>
            ) : (
              <Form.File
                label="Event Class Results"
                custom
                onChange={(event: ChangeEvent<HTMLInputElement>) => {
                  if (event.target.files && event.target.files.length) {
                    this.setState({
                      eventResultsFile: event.target.files[0],
                    });
                  }
                }}
              />
            )}
          </Col>
        </Row>

        <Row>
          <Col>
            <Button
              disabled={this.state.eventResultsFile === undefined}
              variant={'primary'}
              onClick={async () => {
                const results = await this.processor.process(
                  await this.state.eventResultsFile!.text(),
                );
                this.setState({ results });
              }}
            >
              Process
            </Button>
          </Col>
        </Row>

        {this.displayResults()}
      </Container>
    );
  }

  private displayResults(): JSX.Element[] {
    return Object.entries(this.state.results || []).map(
      ([classCategory, categoryResults], index) => (
        <Row key={index}>
          <Col>
            <h2>{classCategory}</h2>
            {this.displayCategoryResults(categoryResults)}
          </Col>
        </Row>
      ),
    );
  }

  private displayCategoryResults(
    categoryResults: ClassCategoryResults,
  ): JSX.Element[] {
    return Object.values(categoryResults)
      .map((classResults) => ({
        ...classResults,
        results: classResults.results.filter(
          (driverResults) => driverResults.times.length,
        ),
      }))
      .filter((classResults) => classResults.results.length)
      .sort((a, b) => {
        if (a.carClass < b.carClass) return -1;
        if (a.carClass > b.carClass) return 1;
        else return 0;
      })
      .map((classResults, index) => (
        <Table key={index} striped hover borderless>
          <thead>
            <tr>
              <th colSpan={10}>
                {classResults.carClass} (Trophies: {classResults.trophyCount})
              </th>
            </tr>
            <tr>
              {ClassResultsProcessor.HEADER.slice(0, 6).map((header, index) => (
                <th key={index}>{header}</th>
              ))}
              <th colSpan={7}>Lap Times</th>
              <th>Fastest</th>
              <th>Difference</th>
            </tr>
          </thead>
          <tbody>{this.displayClassResults(classResults)}</tbody>
        </Table>
      ));
  }

  private displayClassResults(classResults: ClassResults): JSX.Element[] {
    const bestLapInClass = [...classResults.results[0].times].sort(
      LapTime.compare,
    )[0].time;
    return classResults.results.map((driver, index) => {
      const driverBestLap = [...driver.times].sort(LapTime.compare)[0].time;
      return (
        <tr key={index}>
          <td>{driver.trophy ? 'T' : ''}</td>
          <td>{driver.rookie ? 'R' : ''}</td>
          <td>{driver.position}</td>
          <td>{driver.carNumber}</td>
          <td>{driver.name}</td>
          <td>{driver.carDescription}</td>
          {driver.times.map((lapTime, index) => (
            <td key={index}>{App.displayLapTime(lapTime)}</td>
          ))}
          {new Array(7 - driver.times.length).fill(null).map((_, index) => (
            <td key={index} />
          ))}
          <td>{driverBestLap}</td>
          <td>
            {bestLapInClass === driverBestLap
              ? ''
              : `(${(bestLapInClass! - driverBestLap!).toFixed(3)})`}
          </td>
        </tr>
      );
    });
  }

  private static displayLapTime(lapTime: LapTime): string {
    if (lapTime.dnf) {
      return 'DNF';
    } else if (lapTime.rerun) {
      return 'Re-run';
    } else {
      return `${lapTime.time}` + (lapTime.cones ? `(${lapTime.cones})` : '');
    }
  }
}

export default App;
